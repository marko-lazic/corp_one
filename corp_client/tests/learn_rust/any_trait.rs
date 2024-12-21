use std::any::Any;

#[test]
fn any_trait() {
    let cat = FavoritePet { pet: Box::new(Cat) };
    let dog = FavoritePet { pet: Box::new(Dog) };

    let extracted_cat = cat.get::<Cat>().unwrap();
    let extracted_dog = dog.get::<Dog>().unwrap();
    dbg!(extracted_cat);
    dbg!(extracted_dog);
    assert_eq!(extracted_cat.meow(), "Meow");
    assert_eq!(extracted_dog.bark(), "Bark");
}

struct FavoritePet {
    pet: Box<dyn Any>,
}

impl FavoritePet {
    pub fn get<T: Any + 'static>(&self) -> Option<&T> {
        self.pet.downcast_ref()
    }
}

#[derive(Debug)]
struct Dog;

impl Dog {
    pub fn bark(&self) -> &str {
        "Bark"
    }
}

#[derive(Debug)]
struct Cat;

impl Cat {
    pub fn meow(&self) -> &str {
        "Meow"
    }
}
