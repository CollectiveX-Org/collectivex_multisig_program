# CollectiveX Multisig Program

## Overview
The CollectiveX Multisig is a Solana program that enables shared control of accounts through a flexible multisignature scheme. It allows multiple parties to jointly manage assets and execute transactions by requiring a threshold of approvals before any action can be taken.

## Instructions

### Program Configuration

#### program_config_init
Initializes the program-wide configuration.

Parameters:
- `authority`: Public key of the program authority
- `creation_fee`: Fee required to create new multisig accounts
- `treasury`: Public key where fees will be sent

#### program_config_set_authority 
Updates the program authority.

Parameters:
- `new_authority`: New authority public key

#### program_config_set_creation_fee
Updates the fee required to create new multisig accounts.

Parameters:
- `new_creation_fee`: New fee amount in lamports

#### program_config_set_treasury  
Updates the treasury account that receives fees.

Parameters:
- `new_treasury`: New treasury public key

### Multisig Management

#### multisig_create
Creates a new multisig account.

Parameters:
- `config_authority`: Public key that can modify multisig settings
- `threshold`: Number of approvals required to execute transactions
- `members`: Vector of public keys for initial members
- `time_lock`: Minimum time (in seconds) between proposal creation and execution

#### multisig_add_member
Adds a new member to an existing multisig.

Parameters:
- `new_member`: Public key of the member to add

#### multisig_remove_member
Removes a member from the multisig.

Parameters:
- `old_member`: Public key of the member to remove

#### multisig_set_time_lock
Updates the time lock period for the multisig.

Parameters:
- `new_time_lock`: New time lock duration in seconds

#### multisig_set_config_authority
Updates the multisig configuration authority.

Parameters:
- `new_config_authority`: New authority public key

### Spending Limits

#### multisig_add_spending_limit
Adds a spending limit rule to the multisig.

Parameters:
- `vault_index`: Index of the vault account
- `mint`: Token mint address
- `amount`: Maximum amount allowed
- `members`: Vector of members who can approve within this limit
- `destinations`: Vector of allowed destination addresses

#### multisig_remove_spending_limit
Removes a spending limit rule.

Parameters:
- `memo`: Optional note explaining why limit was removed

### Transaction Management

#### config_transaction_create
Creates a new configuration transaction.

Parameters:
- `actions`: Vector of configuration actions to execute

#### proposal_create
Creates a new transaction proposal.

Parameters:
- `transaction_index`: Index of the transaction
- `draft`: Boolean indicating if proposal starts in draft mode

#### proposal_activate
Activates a draft proposal, making it eligible for voting.

#### proposal_approve
Approves a proposal.

Parameters:  
- `memo`: Optional approval note

#### proposal_reject
Rejects a proposal.

Parameters:
- `memo`: Optional rejection note

#### proposal_cancel
Cancels an in-progress proposal.

Parameters:
- `memo`: Optional cancellation note

## Usage Flow

A typical usage flow might look like:

1. Program authority initializes program config with `program_config_init`
2. Users create a new multisig using `multisig_create`
3. Multisig members can be added/removed using `multisig_add_member`/`multisig_remove_member`
4. Optional spending limits can be configured with `multisig_add_spending_limit`
5. Members create proposals using `proposal_create`
6. Draft proposals must be activated with `proposal_activate`
7. Members vote on proposals using `proposal_approve`/`proposal_reject`
8. Proposals can be cancelled if needed using `proposal_cancel`

## Security Considerations

- All authority changes require verification of the current authority
- Time locks prevent rushed execution of proposals 
- Spending limits can restrict transaction amounts and destinations
- Member management requires config authority approval
- Draft proposals allow review before voting begins
- Memos provide audit trail for key actions
