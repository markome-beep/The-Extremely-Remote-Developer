pub trait Poppable {
    type Item;

    fn pop(&mut self) -> Option<Self::Item>;
    fn push(&mut self, item: Self::Item);

    fn pop_2(&mut self) -> Option<(Self::Item, Self::Item)> {
        if let Some(first_pop) = self.pop() {
            if let Some(second_pop) = self.pop() {
                Some((first_pop, second_pop))
            } else {
                self.push(first_pop);
                None
            }
        } else {
            None
        }
    }
}

impl<T> Poppable for Vec<T> {
    type Item = T; // Specify the associated type

    fn pop(&mut self) -> Option<Self::Item> {
        Vec::pop(self)
    }

    fn push(&mut self, item: T) {
        Vec::push(self, item);
    }
}
