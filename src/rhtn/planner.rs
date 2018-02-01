use rhtn::task::*;

const DEBUG : bool = false;

pub struct Domain<'a, T: 'a> {
    pub root_task: Task<'a, T>,
}

#[derive(Clone)]
struct PlannerState<'a, T: 'a> where T: Clone {
    world_state: T,
    method_idx: usize,
    decomp_task: Option<&'a Task<'a, T>>,
    tasks_to_process: Vec<&'a Task<'a, T>>,
    final_plan: Vec<&'a Task<'a, T>>,
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

pub fn generate_plan<'a, T>(domain: &'a Domain<T>, world_state: T) -> Vec<&'a Task<'a, T>> where T: Clone + Copy {
    let mut state = PlannerState::new(world_state);
    let mut planner_stack : Vec<PlannerState<T>> = Vec::new();
    state.tasks_to_process.push(&domain.root_task);

    while let Some(current_task) = state.tasks_to_process.pop() {
        match current_task {
            &Task::Complex(ref ct) => {
                //find satisfied method
                let mut satisfied_method = None;
                while let Some(method) = ct.methods.get(state.method_idx) {
                    if DEBUG { println!("Evaluating {}", state.method_idx); }
                    if (method.condition)(&state.world_state) {
                        if DEBUG { println!("Satisfied"); }
                        satisfied_method = Some(method);
                        break;
                    }else{
                        if DEBUG { println!("Unsatisfied - Stepping"); }
                        state.step();
                    }
                }

                match satisfied_method {
                    Some(method) => {
                        // Record Decomposition of Task
                        planner_stack.push(PlannerState {
                            world_state: state.world_state.clone(),
                            method_idx: state.method_idx,
                            decomp_task: Some(current_task),
                            tasks_to_process: state.tasks_to_process.clone(),
                            final_plan: state.final_plan.clone(),
                        });
                        for sub_task in method.sub_tasks.iter().rev(){
                            state.tasks_to_process.push(sub_task.clone());
                        }
                        state.method_idx = 0;
                    },
                    None => {
                        // Restore to last Decomposed Task
                        if DEBUG { println!("No satisfiable methods, rewinding"); }
                        if let Some(pop) = planner_stack.pop() {
                            state = pop;
                            if let Some(decomp_task) = state.decomp_task {
                                state.tasks_to_process.push(decomp_task);
                                state.step();
                            }else{ assert!(false); } //shouldn't happen
                        }else{ assert!(false); } //shouldn't happen
                    }
                }
            },
            &Task::Primitive(ref pt) => {
                if (pt.condition)(&state.world_state){
                    if DEBUG { println!("Can accomplish primitive {}, applying", pt.name); }
                    (pt.effect)(&mut state.world_state);
                    state.final_plan.push(current_task);
                }else{
                    if DEBUG { println!("Cannot accomplish primitive task {}, rewinding", pt.name); }
                    // Restore to last Decomposed Task
                    if let Some(pop) = planner_stack.pop() {
                        state = pop;
                        if let Some(decomp_task) = state.decomp_task {
                            state.tasks_to_process.push(decomp_task);
                            state.step();
                        }else{ assert!(false); } //shouldn't happen
                    }else{ assert!(false); } //shouldn't happen
                }
            },
        }
    }

    return state.final_plan;
}