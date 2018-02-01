use rhtn::task::*;

pub struct Domain<'a, T: 'a> {
    pub root_task: ComplexTask<'a, T>,
}

#[derive(Clone)]
struct PlannerState<'a, T: 'a> where T: Clone {
    world_state: T,
    method_idx: usize,
    decomp_task: Option<&'a ComplexTask<'a, T>>,
    tasks_to_process: Vec<Task<'a, T>>,
    final_plan: Vec<&'a PrimitiveTask<'a, T>>,
}

impl<'a, T: 'a> PlannerState<'a, T> where T: Clone {
    fn step(&mut self){
        self.method_idx += 1;
    }

    fn new(world_state: T) -> PlannerState<'a, T>{
        PlannerState {
            world_state: world_state.clone(),
            method_idx: 0,
            decomp_task: None,
            tasks_to_process: Vec::new(),
            final_plan: Vec::new(),
        }
    }
}

pub fn generate_plan<'a, T>(domain: &'a Domain<T>, world_state: T) -> Vec<&'a PrimitiveTask<'a, T>> where T: Clone + Copy {
    let root_task = Task::Complex(&domain.root_task);
    let mut ps = PlannerState::new(world_state);
    let mut planner_stack : Vec<PlannerState<T>> = Vec::new();
    ps.tasks_to_process.push(root_task);

    while let Some(current_task) = ps.tasks_to_process.pop() {
        match current_task {
            Task::Complex(ct) => {

                //find satisfied method
                let mut satisfied_method = None;
                while let Some(method) = ct.methods.get(ps.method_idx) {
                    println!("Evaluating {}", ps.method_idx);
                    if (method.condition)(&ps.world_state) {
                        println!("Satisfied");
                        satisfied_method = Some(method);
                        break;
                    }else{
                        println!("Unsatisfied - Stepping");
                        ps.step();
                    }
                }

                match satisfied_method {
                    Some(method) => {
                        // Record Decomposition of Task
                        planner_stack.push(PlannerState {
                            world_state: ps.world_state.clone(),
                            method_idx: ps.method_idx,
                            decomp_task: Some(ct),
                            tasks_to_process: ps.tasks_to_process.clone(),
                            final_plan: ps.final_plan.clone(),
                        });
                        for sub_task in method.sub_tasks.iter().rev(){
                            ps.tasks_to_process.push(sub_task.clone());
                        }
                        ps.method_idx = 0;
                    },
                    None => {
                        // Restore to last Decomposed Task
                        println!("No satisfiable methods, rewinding");
                        if let Some(pop) = planner_stack.pop() {
                            ps = pop;
                            if let Some(decomp_task) = ps.decomp_task {
                                ps.tasks_to_process.push(Task::Complex(decomp_task));
                                ps.step();
                            }else{ assert!(false); } //shouldn't happen
                        }else{ assert!(false); } //shouldn't happen
                    }
                }
            },
            Task::Primitive(pt) => {
                if (pt.condition)(&ps.world_state){
                    println!("Can accomplish primitive {}, applying", pt.name);
                    (pt.effect)(&mut ps.world_state);
                    ps.final_plan.push(&pt);
                }else{
                    println!("Cannot accomplish primitive task {}, rewinding", pt.name);
                    // Restore to last Decomposed Task
                    if let Some(pop) = planner_stack.pop() {
                        ps = pop;
                        if let Some(decomp_task) = ps.decomp_task {
                            ps.tasks_to_process.push(Task::Complex(decomp_task));
                            ps.step();
                        }else{ assert!(false); } //shouldn't happen
                    }else{ assert!(false); } //shouldn't happen
                }
            },
        }
    }

    return ps.final_plan;
}