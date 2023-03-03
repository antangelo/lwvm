#[derive(Debug, Clone, Copy)]
struct RegisterMapping {
    offset: usize,
}

pub struct Context<StateType> {
    reg_map: [Option<RegisterMapping>; 256],

    phantom: core::marker::PhantomData<StateType>,
}

impl<T> Default for Context<T> {
    fn default() -> Self {
        Self {
            reg_map: [None; 256],

            phantom: core::marker::PhantomData,
        }
    }
}

impl<T> Context<T> {
    pub fn map_register(&mut self, register: u8, offset: usize) {
        if offset >= core::mem::size_of::<T>() {
            panic!("Offset {} is greater than size of state type", offset);
        }

        self.reg_map[register as usize] = Some(RegisterMapping { offset });
    }

    pub fn new_unit<'ctx>(&'ctx mut self) -> TranslationUnit<'ctx, T> {
        TranslationUnit::new(self)
    }
}

pub(crate) struct CompiledUnitHandle<StateType> {
    state: StateType,
}

pub struct TranslationUnit<'ctx, StateType> {
    context: &'ctx Context<StateType>,
    compiled: Option<CompiledUnitHandle<StateType>>,
}

impl<'ctx, T> TranslationUnit<'ctx, T> {
    fn new(context: &'ctx mut Context<T>) -> Self {
        Self {
            context,
            compiled: None,
        }
    }
}
