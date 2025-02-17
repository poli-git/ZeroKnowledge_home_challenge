//! Generated crate containing the image ID and ELF binary of the build guest.
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

#[cfg(test)]
mod tests {
    use alloy_primitives::U256;
    use alloy_sol_types::SolValue;
    use risc0_zkvm::{default_executor, ExecutorEnv};

    #[test]
    fn proves_odd_number() {
        let odd_number = U256::from(10201);

        let env = ExecutorEnv::builder()
            .write_slice(&odd_number.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        let session_info = default_executor().execute(env, super::IS_ODD_ELF).unwrap();

        let x = U256::abi_decode(&session_info.journal.bytes, true).unwrap();
        assert_eq!(x, odd_number);
    }

    #[test]
    #[should_panic(expected = "number is not odd")]
    fn rejects_even_number() {
        let even_number = U256::from(12);

        let env = ExecutorEnv::builder()
            .write_slice(&even_number.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        default_executor().execute(env, super::IS_ODD_ELF).unwrap();
    }
}
