extern crate bellman;
extern crate rand;

use bellman::groth16::{
    create_random_proof, prepare_verifying_key, verify_proof, Proof, VerifyingKey,
};
use bellman::pairing::bn256::{Bn256, Fr};
use bellman::Circuit;
use rand::RngCore;

pub struct ZKPProof {
    proof: Proof<Bn256>,
}

impl ZKPProof {
    pub fn generate_proof<R: RngCore, C: Circuit<Bn256>>(
        circuit: C,
        rng: &mut R,
        vote_choice: u32,
        private_data: &[u8],
    ) -> Self {
        // Generate parameters for the circuit
        let params =
            bellman::groth16::generate_random_parameters::<Bn256, _, _>(circuit, rng).unwrap();

        // Create a proof
        let proof = create_random_proof::<Bn256, _, _>(
            circuit,
            &params,
            rng,
        )
        .expect("Failed to create proof");

        ZKPProof { proof }
    }

    pub fn verify_proof(&self, public_data: &[u8]) -> bool {
        let public_fr = Fr::from(public_data); // Convert public data to Fr element
        let vk = prepare_verifying_key(&self.proof.vk);

        verify_proof::<Bn256>(
            &vk,
            &self.proof,
            |c| {
                let public_var = Some(public_fr);
                
                Ok(public_var)
            },
        )
        .expect("Proof verification failed")
    }
}
