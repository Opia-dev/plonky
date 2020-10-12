use std::marker::PhantomData;

use crate::gates::gate_collection::{GateCollection, GatePrefixes};
use crate::gates::Gate;
use crate::{CircuitBuilder, HaloCurve, PartialWitness, Target, WitnessGenerator};

/// A gate which doesn't perform any arithmetic, but just acts as a buffer for receiving data.
/// Some gates, such as the Rescue round gate, "output" their results using one of the next gate's
/// "input" wires. The last such gate has no next gate of the same type, so we add a buffer gate
/// for receiving the last gate's output.
pub struct BufferGate<C: HaloCurve> {
    pub index: usize,
    _phantom: PhantomData<C>,
}

impl<C: HaloCurve> BufferGate<C> {
    pub fn new(index: usize) -> Self {
        BufferGate {
            index,
            _phantom: PhantomData,
        }
    }
}

impl<C: HaloCurve> Gate<C> for BufferGate<C> {
    fn name(&self) -> &'static str {
        "BufferGate"
    }
    fn degree(&self) -> usize {
        0
    }
    fn num_constants(&self) -> usize {
        0
    }

    fn evaluate_unfiltered(
        &self,
        gates: &GateCollection<C>,
        _local_constant_values: &[C::ScalarField],
        _local_wire_values: &[C::ScalarField],
        _right_wire_values: &[C::ScalarField],
        _below_wire_values: &[C::ScalarField],
    ) -> Vec<C::ScalarField> {
        Vec::new()
    }

    fn evaluate_unfiltered_recursively(
        &self,
        _builder: &mut CircuitBuilder<C>,
        gates: &GateCollection<C>,
        _local_constant_values: &[Target<C::ScalarField>],
        _local_wire_values: &[Target<C::ScalarField>],
        _right_wire_values: &[Target<C::ScalarField>],
        _below_wire_values: &[Target<C::ScalarField>],
    ) -> Vec<Target<C::ScalarField>> {
        Vec::new()
    }
}

impl<C: HaloCurve> WitnessGenerator<C::ScalarField> for BufferGate<C> {
    fn dependencies(&self) -> Vec<Target<C::ScalarField>> {
        Vec::new()
    }

    fn generate(
        &self,
        prefixes: &GatePrefixes,
        _constants: &[Vec<C::ScalarField>],
        _witness: &PartialWitness<C::ScalarField>,
    ) -> PartialWitness<C::ScalarField> {
        PartialWitness::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_gate_low_degree, BufferGate, Tweedledee, Tweedledum};

    test_gate_low_degree!(
        low_degree_BufferGate,
        Tweedledum,
        Tweedledee,
        BufferGate<Tweedledum>
    );
}
