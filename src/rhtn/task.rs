

pub struct ComplexTask<'a, T: 'a> {
    pub methods: Vec<Method<'a, T>>
}

pub struct Method<'a, T: 'a> {
    pub condition: fn(&T) -> bool,
    pub sub_tasks: Vec<Task<'a, T>>,
}

pub struct PrimitiveTask<T> {
    pub name: String,
    pub condition: fn(&T) -> bool,
    pub effect: fn(&mut T),
    pub action: fn(),
}

#[derive(Copy, Clone)]
pub enum Task<'a, T: 'a> {
    Complex (&'a ComplexTask<'a, T>),
    Primitive (&'a PrimitiveTask<T>),
}
