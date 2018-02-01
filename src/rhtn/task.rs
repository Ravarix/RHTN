

pub struct ComplexTask<'a, T: 'a> {
    pub methods: Vec<Method<'a, T>>
}

pub struct Method<'a, T: 'a> {
    pub condition: fn(&T) -> bool,
    pub sub_tasks: Vec<&'a Task<'a, T>>,
}

pub struct PrimitiveTask<'a, T> {
    pub name: String,
    pub condition: Box<Fn(&T) -> bool + 'a>,
    pub effect: Box<Fn(&mut T) + 'a>,
    pub action: fn(),
}

pub enum Task<'a, T: 'a> {
    Complex (ComplexTask<'a, T>),
    Primitive (PrimitiveTask<'a, T>),
}

pub struct ArgTask<T, TArgs> {
    pub name: String,
    pub condition: fn(&T, TArgs) -> bool,
    pub effect: fn(&mut T, TArgs),
    pub action: fn(),
}

impl <T, TArgs> ArgTask<T, TArgs> where TArgs : Copy {
    pub fn with(&self, args: TArgs) -> Task<T> {
        Task::Primitive( PrimitiveTask {
            name: self.name.clone(),
            condition: Box::new(move |ws| (self.condition)(ws, args)),
            effect: Box::new(move |mut ws| (self.effect)(&mut ws, args)),
            action: self.action,
        })
    }
}