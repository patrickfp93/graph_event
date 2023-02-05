#[cfg(test)]
mod always_conditon;

pub trait Value<T>{
    fn get_mut(&mut self) -> &mut T;

    fn get(&self) -> &T;
}

pub struct A(usize);

impl Value<usize> for A {
    fn get_mut (&mut self) -> &mut usize {
        &mut self.0
    }

    fn get(&self) -> &usize {
        &self.0
    }
}

pub struct B(usize);

impl Value<usize> for B {
    fn get_mut (&mut self) -> &mut usize {
        &mut self.0
    }
    fn get(&self) -> &usize {
        &self.0
    }

}

