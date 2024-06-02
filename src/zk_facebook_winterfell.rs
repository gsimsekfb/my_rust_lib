use winterfell::{
    math::{fields::f128::BaseElement, FieldElement, ToElements},
    TraceTable,
};

use winterfell::{
    Air, AirContext, Assertion, EvaluationFrame, ProofOptions, TraceInfo,
    TransitionConstraintDegree,
};

// Contents:
// 0. How a user will use our START impl.: prove_work() and verify_work()
// 1. Why use STARK
// 2. Steps to implement STARK:

// todo:
// copy readme

// --------------------------------------------------------------------------

// 0. How a user will use our START impl.: prove_work() and verify_work()

#[test]
fn ex1_zk() {
    let (result, proof) = prove_work();
    println!("res: {:?}", result);
        // 190393255176150493381245531460827183000
    let start = winterfell::math::fields::f128::BaseElement::new(3);
    
    assert_eq!(verify_work(start, result, proof.clone()), true);
    // Verify should fail with wrong result
    assert_eq!(verify_work(start, result.double(), proof), false);
}

use winterfell::{
    FieldExtension, Proof,
};

// Generate a STARK proof
// The function below, will execute our computation, and will return the result
// together with the proof that the computation was executed correctly
pub fn prove_work() -> (BaseElement, Proof) {
    // We'll just hard-code the parameters here for this example.
    let start = BaseElement::new(3);
    let n = 8;
    // let n = 1_048_576; // orig; use this for long computation

    // Build the execution trace and get the result from the last step.
    let trace = build_do_work_trace(start, n);
    let result = trace.get(0, n - 1);

    // Define proof options; these will be enough for ~96-bit security level.
    let options = ProofOptions::new(
        32, // number of queries
        8,  // blowup factor
        0,  // grinding factor
        FieldExtension::None,
        8,   // FRI folding factor
        127, // FRI remainder max degree
    );

    // Instantiate the prover and generate the proof.
    let prover = WorkProver::new(options);
    let proof = prover.prove(trace).unwrap();

    (result, proof)
}


use winterfell::{
    verify, AcceptableOptions, 
};

// type Blake3 = Blake3_256<BaseElement>;

// Verify work/proof
// We give this proof and the public inputs to anyone, and they can verify 
// that we did in fact execute the computation and got the claimed result. 
pub fn verify_work(
    start: BaseElement, result: BaseElement, proof: Proof
) -> bool {
    // The verifier will accept proofs with parameters which guarantee 95 bits 
    // or more of conjectured security
    let min_opts = AcceptableOptions::MinConjecturedSecurity(95);

    // The number of steps and options are encoded in the proof itself,
    // so we don't need to pass them explicitly to the verifier.
    let pub_inputs = PublicInputs { start, result };
    verify::<WorkAir, Blake3, DefaultRandomCoin<Blake3>>(
        proof, pub_inputs, &min_opts
    ).is_ok()
}

// --------------------------------------------------------------------------


// 1. Why use STARK (Scalable Transparent Argument of Knowledge) ?
// Suppose, we run this computation for a million steps and get some result. 
// Using STARKs we can prove that we did the work correctly without requiring 
// any verifying party to re-execute the computation

// The long computation to generate and verify STARK proofs
fn do_work(start: BaseElement, n: usize) -> BaseElement {
    let mut result = start;
    for _ in 1..n {
        result = result.exp(3) + BaseElement::new(42);
    }
    result
}


// --------------------------------------------------------------------------


// 2. Steps to implement STARK:
// 2.a. Define an execution trace
// 2.b. Arithmetization
// 2.c. Implement Prover trait

// -----------------------------------

// 2.a. Define an execution trace
// we need to define an execution trace for our computation. 
// This trace should capture the state of the computation at every step of its 
// execution. In our case, the trace is just a single column of intermediate 
// values after each execution of the loop.
//
// e.g.
// For start: BaseElement(3), n: 1,048,575
//
// Step	        State
// 0	        3
// 1	        69
// 2	        328551
// 3	        35465687262668193
// 4	        237280320818395402166933071684267763523
// ...	
// 1,048,575	247770943907079986105389697876176586605

// Just a modified version of the do_work() function which records every 
// intermediate state of the computation 
pub fn build_do_work_trace(start: BaseElement, n: usize) -> TraceTable<BaseElement> {
    // Instantiate the trace with a given width and length; this will allocate all
    // required memory for the trace
    let trace_width = 1;
    let mut trace = TraceTable::new(trace_width, n);

    // Fill the trace with data
    trace.fill(
        |state| {    // initialize the first state
            state[0] = start;
        },
        |_, state| { // compute the next state
            state[0] = state[0].exp(3u32.into()) + BaseElement::new(42);
        },
    );

    trace
}

// -----------------------------------

// 2.b. Arithmetization
// Define algebraic intermediate representation (AIR) for our computation
// aka arithmetization

// We do this by implementing the Air trait. At the high level, the code below 
// does three things:
// i- Defines what the public inputs for our computation should look like.
// These inputs are called "public" because they must be known to both, 
// the prover and the verifier.

// ii- Defines a transition function with a single transition constraint. 
// This transition constraint must evaluate to zero for all valid state 
// transitions, and to non-zero for any invalid state transition. 
// The degree of this constraint is 3 (see more about constraint degrees here:
// https://github.com/facebook/winterfell/tree/main/air#Constraint-degrees)

// iii- Define two assertions against an execution trace of our computation.
// These assertions tie a specific set of public inputs to a specific execution
// trace (see more about assertions here:
// https://github.com/facebook/winterfell/tree/main/air#Trace-assertions)

// Public inputs for our computation are: (starting value and end result)
pub struct PublicInputs {
    start: BaseElement,
    result: BaseElement,
}

// We need to describe how public inputs can be converted to *field elements.
// * field element: an element in a finite field
impl ToElements<BaseElement> for PublicInputs {
    fn to_elements(&self) -> Vec<BaseElement> {
        vec![self.start, self.result]
    }
}

// For a specific instance of our computation, we'll keep track of the public 
// inputs and the computation's context which we'll build in the constructor. 
// The context is used internally by the Winterfell prover/verifier when 
// interpreting this AIR.
pub struct WorkAir {
    context: AirContext<BaseElement>,
        // STARK params and trace properties for a specific exec. of a computation.
    start: BaseElement,
    result: BaseElement,
}

// todo: 1st complex section
// Note: 
// AIR TRAIT: Describes algebraic intermediate representation of a computation.
impl Air for WorkAir {
    // First, we'll specify which finite field to use for our computation, and also how
    // the public inputs must look like.
    type BaseField = BaseElement;
    type PublicInputs = PublicInputs;
    // gg
    type GkrProof = ();
    type GkrVerifier = ();

    // Here, we'll construct a new instance of our computation which is defined by 3 parameters:
    // starting value, number of steps, and the end result. Another way to think about it is
    // that an instance of our computation is a specific invocation of the do_work() function.
    fn new(trace_info: TraceInfo, pub_inputs: PublicInputs, options: ProofOptions) -> Self {
        // our execution trace should have only one column.
        assert_eq!(1, trace_info.width());

        // Our computation requires a single transition constraint. The constraint itself
        // is defined in the evaluate_transition() method below, but here we need to specify
        // the expected degree of the constraint. If the expected and actual degrees of the
        // constraints don't match, an error will be thrown in the debug mode, but in release
        // mode, an invalid proof will be generated which will not be accepted by any verifier.
        let degrees = vec![TransitionConstraintDegree::new(3)];

        // We also need to specify the exact number of assertions we will place against the
        // execution trace. This number must be the same as the number of items in a vector
        // returned from the get_assertions() method below.
        let num_assertions = 2;

        WorkAir {
            context: AirContext::new(trace_info, degrees, num_assertions, options),
            start: pub_inputs.start,
            result: pub_inputs.result,
        }
    }

    // In this method we'll define our transition constraints; a computation is considered to
    // be valid, if for all valid state transitions, transition constraints evaluate to all
    // zeros, and for any invalid transition, at least one constraint evaluates to a non-zero
    // value. The `frame` parameter will contain current and next states of the computation.
    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        // First, we'll read the current state, and use it to compute the expected next state
        let current_state = &frame.current()[0];
        let next_state = current_state.exp(3u32.into()) + E::from(42u32);

        // Then, we'll subtract the expected next state from the actual next state; this will
        // evaluate to zero if and only if the expected and actual states are the same.
        result[0] = frame.next()[0] - next_state;
    }

    // Here, we'll define a set of assertions about the execution trace which must be satisfied
    // for the computation to be valid. Essentially, this ties computation's execution trace
    // to the public inputs.
    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        // for our computation to be valid, value in column 0 at step 0 must be equal to the
        // starting value, and at the last step it must be equal to the result.
        let last_step = self.trace_length() - 1;
        vec![
            Assertion::single(0, 0, self.start),
            Assertion::single(0, last_step, self.result),
        ]
    }

    // This is just boilerplate which is used by the Winterfell prover/verifier to retrieve
    // the context of the computation.
    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }
}


// -----------------------------------


// 2.c. Implement Prover trait

use winterfell::{
    AuxRandElements,
    crypto::{hashers::Blake3_256, DefaultRandomCoin},
    matrix::ColMatrix,
    DefaultConstraintEvaluator, DefaultTraceLde, Prover, StarkDomain, Trace,
    TracePolyTable,
};

// We'll use BLAKE3 as the hash function during proof generation.
type Blake3 = Blake3_256<BaseElement>;

// Our prover needs to hold STARK protocol parameters which are specified via ProofOptions
// struct.
struct WorkProver {
    options: ProofOptions,
}

impl WorkProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }
}

// Implement Prover trait
//
// When implementing the Prover trait we set the `Air` associated type to the AIR of the
// computation we defined previously, and set the `Trace` associated type to `TraceTable`
// struct as we don't need to define a custom trace for our computation. For other
// associated types, we'll use default implementation provided by Winterfell.
impl Prover for WorkProver {
    type BaseField = BaseElement;
    type Air = WorkAir;
    type Trace = TraceTable<BaseElement>;
    type HashFn = Blake3;
    type RandomCoin = DefaultRandomCoin<Blake3>;
    type TraceLde<E: FieldElement<BaseField = BaseElement>> = DefaultTraceLde<E, Blake3>;
    type ConstraintEvaluator<'a, E: FieldElement<BaseField = BaseElement>> =
        DefaultConstraintEvaluator<'a, WorkAir, E>;

    // Our public inputs consist of the first and last value in the execution trace.
    fn get_pub_inputs(&self, trace: &Self::Trace) -> PublicInputs {
        let last_step = trace.length() - 1;
        PublicInputs {
            start: trace.get(0, 0),
            result: trace.get(0, last_step),
        }
    }

    // We'll use the default trace low-degree extension.
    fn new_trace_lde<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        trace_info: &TraceInfo,
        main_trace: &ColMatrix<Self::BaseField>,
        domain: &StarkDomain<Self::BaseField>,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
        DefaultTraceLde::new(trace_info, main_trace, domain)
    }

    // We'll use the default constraint evaluator to evaluate AIR constraints.
    fn new_evaluator<'a, E: FieldElement<BaseField = BaseElement>>(
        &self,
        air: &'a WorkAir,
        aux_rand_elements: Option<AuxRandElements<E>>,
        composition_coefficients: winterfell::ConstraintCompositionCoefficients<E>,
    ) -> Self::ConstraintEvaluator<'a, E> {
        DefaultConstraintEvaluator::new(air, aux_rand_elements, composition_coefficients)
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}

// End 
