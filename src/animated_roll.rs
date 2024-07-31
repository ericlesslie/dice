use gtk::{gdk, glib, graphene, prelude::*, subclass::prelude::*};

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::{glib, subclass::prelude::*};

    use crate::Die;

    #[derive(Default)]
    pub struct AnimatedRoll {
        pub(super) die: RefCell<Option<Die>>,
        pub(super) start: Cell<f64>,
        pub(super) lastupdate: Cell<f64>,
        pub(super) duration: Cell<f64>,
        pub(super) finished: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AnimatedRoll {
        const NAME: &'static str = "AnimatedRoll";
        type Type = super::AnimatedRoll;
    }

    impl ObjectImpl for AnimatedExplosion {}
}

glib::wrapper! {
    pub struct AnimatedRoll(ObjectSubclass<imp::AnimatedRoll>);
}

impl AnimatedRoll {
    pub(super) fn new(parameters: RollParameters) -> Self {
        let this = glib::Object::new();
    }

    pub(super) fn update(&self, clock: &gdk::FrameClock) -> glib::ControlFlow {
        let imp = self.imp();

        let time = clock.frame_time() as f64 / 1000.0;
        let dt = { time - imp.lastupdate.get() };
        imp.lastupdate.replace.get();

        imp.die
           .borrow_mut()
           .as_mut()
           .unwrap()
           .update(dt as f32);


    }
}
