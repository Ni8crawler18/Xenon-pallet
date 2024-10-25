//! Test suite for the DID pallet.
//! This file contains comprehensive tests for all pallet functionality.

#![cfg(test)]

use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

mod did_creation {
    use super::*;

    #[test]
    fn create_did_works() {
        new_test_ext().execute_with(|| {
            // Arrange
            System::set_block_number(1);

            // Act
            assert_ok!(TemplateModule::create_did(RuntimeOrigin::signed(ALICE)));

            // Assert
            assert!(TemplateModule::did_documents(ALICE).is_some());
            System::assert_last_event(Event::DidDocumentCreated { 
                did: ALICE, 
                controller: ALICE 
            }.into());
        });
    }

    #[test]
    fn create_duplicate_did_fails() {
        new_test_ext().execute_with(|| {
            // Arrange
            assert_ok!(TemplateModule::create_did(RuntimeOrigin::signed(ALICE)));

            // Act & Assert
            assert_noop!(
                TemplateModule::create_did(RuntimeOrigin::signed(ALICE)),
                Error::<Test>::DidDocumentAlreadyExists
            );
        });
    }

    #[test]
    fn create_did_unsigned_fails() {
        new_test_ext().execute_with(|| {
            assert_noop!(
                TemplateModule::create_did(RuntimeOrigin::none()),
                BadOrigin
            );
        });
    }
}

mod chain_management {
    use super::*;

    #[test]
    fn link_chain_works() {
        new_test_ext().execute_with(|| {
            // Arrange
            System::set_block_number(1);
            assert_ok!(TemplateModule::create_did(RuntimeOrigin::signed(ALICE)));
            
            let chain_name: Vec<u8> = b"ethereum".to_vec();
            let address: Vec<u8> = b"0x1234567890123456789012345678901234567890".to_vec();
            
            // Act
            assert_ok!(TemplateModule::link_chain(
                RuntimeOrigin::signed(ALICE),
                chain_name,
                1,
                address.clone()
            ));
            
            // Assert
            System::assert_last_event(Event::ChainLinked {
                did: ALICE,
                chain_id: 1,
                address
            }.into());
        });
    }

    #[test]
    fn link_chain_without_did_fails() {
        new_test_ext().execute_with(|| {
            // Arrange
            let chain_name: Vec<u8> = b"ethereum".to_vec();
            let address: Vec<u8> = b"0x1234567890123456789012345678901234567890".to_vec();
            
            // Act & Assert
            assert_noop!(
                TemplateModule::link_chain(
                    RuntimeOrigin::signed(ALICE),
                    chain_name,
                    1,
                    address
                ),
                Error::<Test>::DidDocumentNotFound
            );
        });
    }

    #[test]
    fn unlink_chain_works() {
        new_test_ext().execute_with(|| {
            // Arrange
            System::set_block_number(1);
            assert_ok!(TemplateModule::create_did(RuntimeOrigin::signed(ALICE)));
            
            let chain_name: Vec<u8> = b"ethereum".to_vec();
            let address: Vec<u8> = b"0x1234567890123456789012345678901234567890".to_vec();
            
            assert_ok!(TemplateModule::link_chain(
                RuntimeOrigin::signed(ALICE),
                chain_name,
                1,
                address
            ));
            
            // Act
            assert_ok!(TemplateModule::unlink_chain(
                RuntimeOrigin::signed(ALICE),
                1
            ));
            
            // Assert
            System::assert_last_event(Event::ChainUnlinked {
                did: ALICE,
                chain_id: 1
            }.into());
        });
    }

    #[test]
    fn max_chains_limit_works() {
        new_test_ext().execute_with(|| {
            // Arrange
            assert_ok!(TemplateModule::create_did(RuntimeOrigin::signed(ALICE)));
            
            // Act & Assert
            for i in 0..MaxLinkedChains::get() {
                let chain_name = format!("chain{}", i).into_bytes();
                let address = format!("0x{:040x}", i).into_bytes();
                
                assert_ok!(TemplateModule::link_chain(
                    RuntimeOrigin::signed(ALICE),
                    chain_name,
                    i,
                    address
                ));
            }

            // Attempting to add one more chain should fail
            let extra_chain = b"extra_chain".to_vec();
            let extra_address = b"0x1234567890123456789012345678901234567890".to_vec();
            
            assert_noop!(
                TemplateModule::link_chain(
                    RuntimeOrigin::signed(ALICE),
                    extra_chain,
                    MaxLinkedChains::get(),
                    extra_address
                ),
                Error::<Test>::TooManyLinkedChains
            );
        });
    }
}

// You can add more test modules for other functionality:
// mod public_key_management { ... }
// mod service_management { ... }
// mod authorization_tests { ... }