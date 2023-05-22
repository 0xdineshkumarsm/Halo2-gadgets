use std::marker::PhantomData;
use halo2_proofs::{plonk::{Error,TableColumn, ConstraintSystem},arithmetic::FieldExt, circuit::{Layouter, Value}};

/// A lookup table of values upto RANGE 
/// eg RANGE = 8, values = [0..255]
/// 
/// 

#[derive(Debug,Clone)]
pub(super) struct RangeCheckTable<F:FieldExt, const RANGE:usize>{
    pub(super) value: TableColumn,
    _marker: PhantomData<F>
}

impl<F:FieldExt, const RANGE:usize> RangeCheckTable<F,RANGE> {

    pub(super) fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        let value = meta.lookup_table_column();

        Self { value, _marker: PhantomData }
    }

    pub(super) fn load(&self,layouter:&mut impl Layouter<F>) -> Result<(),Error> {
        layouter.assign_table(|| "load range-check table", |mut table|{
            let mut offset = 0;
            for i in 0..RANGE{
                table.assign_cell(|| "assign cell", self.value, offset,|| Value::known(F::from(i as u64)))?;
            }
            Ok(())
        })
    }
}