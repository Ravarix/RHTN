//use std::env::Args;
mod rhtn;
use rhtn::*;

#[derive(Debug, Copy, Clone)]
struct DemoWorldState {
    cash: i32,
    hunger: u32,
    food: bool,
    hour: u32,
    bank: i32,
}

fn main() {
    let buy_food: Task<DemoWorldState> = Task::Primitive( PrimitiveTask {
        name: "buy food".to_string(),
        condition: Box::new(|ws| ws.cash >= 5),
        effect: Box::new(|ws| {
            ws.cash -= 5;
            ws.food = true;
        }),
        action: ||{},
    });

    let eat_food: Task<DemoWorldState> = Task::Primitive( PrimitiveTask {
        name: "eat food".to_string(),
        condition: Box::new(|ws| ws.food),
        effect: Box::new(|ws| {
            ws.food = false;
            ws.hunger = 0;
        }),
        action: ||{},
    });

    let _work: Task<DemoWorldState> = Task::Primitive( PrimitiveTask {
        name: "work".to_string(),
        condition: Box::new(|_| true),
        effect: Box::new(|ws| {
            ws.bank += 2;
            ws.hunger += 1;
        }),
        action: ||{},
    });

    let pass_time: ArgTask<DemoWorldState, (u32, i32)> = ArgTask {
        name: "pass_time".to_string(),
        condition: |_, _| true,
        effect: |ws, args| {
            ws.hunger += args.0;
            ws.bank += args.1;
        },
        action: ||{},
    };

    let work = pass_time.with((1, 2));
    let game = pass_time.with((1, 0));

    let withdraw_vargd: VarArgTask<DemoWorldState, (i32)> = VarArgTask {
        condition: |ws, amnt| ws.bank >= amnt,
        effect: |ws, amnt| {
            ws.bank -= amnt;
            ws.cash += amnt;
        },
        action: ||{},
    };

    let withdraw_all = withdraw_vargd.with("withdraw_all", |ws| ws.bank );

    let _game: Task<DemoWorldState> = Task::Primitive( PrimitiveTask {
        name: "game".to_string(),
        condition: Box::new(|_| true),
        effect: Box::new(|ws| {
            ws.hunger += 1;
        }),
        action: ||{},
    });


    let cure_hunger: Task<DemoWorldState> = Task::Complex( ComplexTask {
        methods: vec![
            Method {
                condition: |ws| ws.food == true,
                sub_tasks: vec![&eat_food],
            },
            Method {
                condition: |ws| ws.cash >= 5,
                sub_tasks: vec![&buy_food, &eat_food],
            },
            Method {
                condition: |_| true,
                sub_tasks: vec![&withdraw_all, &buy_food, &eat_food]
            }
        ],
    });

    let exist: Task<DemoWorldState> = Task::Complex( ComplexTask {
        methods: vec![
            Method {
                condition: |ws| ws.hunger >= 5,
                sub_tasks: vec![&cure_hunger],
            },
            Method {
                condition: |ws| ws.hour >= 9 && ws.hour <= 17,
                sub_tasks: vec![&work],
            },
            Method {
                condition: |_| true,
                sub_tasks: vec![&game],
            }
        ],
    });

    let d1 = Domain { root_task : exist };

    let mut world = DemoWorldState { cash: 0, bank: 5, hunger: 5, food: false, hour: 0};

    let steps = 24;
    for _ in 0..steps {
        let plan = generate_plan(&d1, world);

        println!("Result plan:");
        let mut delim = "";
        for task in plan {
            match task {
                &Task::Primitive(ref pt) => {
                    print!("{} {}", delim, pt.name);
                    delim = ",";
                    (pt.effect)(&mut world); //apply the effects
                }
                _ => {}
            }
        }
        world.hour = (world.hour + 1) % 24;
        println!("\nAdvancing clock to {}", world.hour);
    }
}
