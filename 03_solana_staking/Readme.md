# Solana Staking Project
## Requirement
```
Let's work on a solana staking application. A user can stake/unstake/restake, withdraw, claim rewards.
An owner can set/change the APY, unbonding period.

So basically user functions:
stake()
unstake() -> Unbonding period will start once user unstakes
withdraw() -> User can only withdraw funds once unbonding period is complete after unstaking()
claimRewards() -> User can claim the rewards accumulated till that time. 

User can restake() or stake more tokens

Things we lookout for:
1.) Clean code
2.) Test coverage, we prefer more than 95% test coverage
3.) Handling of edge cases

```
## 1. Create Project
```
$ anchor init solana-staking
$ cd solana-staking
$ anchor build
```