<div align="center">
    <hr />
    <img src="../.github/readme/ethsecurity.jpg" alt="EthSecurity" />
    <hr />
</div>

### 👀 Overview

This Docker container provides a comprehensive suite of security tools and resources designed for Ethereum smart contract analysis and auditing.

```
 ____ ____ ____ ____ ____ ____ ____ ____ ____ 
||⟠ |||S |||e |||c |||u |||r |||i |||t |||y ||
||__|||__|||__|||__|||__|||__|||__|||__|||__||
|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|

  Ξ https://github.com/cosasdepuma/totes/ Ξ

Resources:
----------------------------------------------
⟠ https://github.com/crytic/crytic-compile
⟠ https://github.com/foundry-rs/foundry
⟠ https://github.com/tj/n
⟠ https://github.com/crytic/solc-select
⟠ https://github.com/vyperlang/vyper

Tools:
----------------------------------------------
⟠ https://github.com/crytic/medusa
⟠ https://github.com/consensys/mythril
⟠ https://github.com/crytic/pyevmasm
⟠ https://github.com/crytic/slither
⟠ https://github.com/crytic/tealer

Notes:
----------------------------------------------
⬨ Use `n` to switch between different versions of `node`
⬨ Use `solc-select` to switch between different versions of `solc`
```

---

### 📖 How to use it

```bash
docker run --name ethsecurity --rm -it -v "$(pwd):/code" cosasdepuma/ethsecurity:latest
```

---

### 🧰 Resources

| Resource | Description |
|----------|-------------|
| [crytic-compile](https://github.com/crytic/crytic-compile) | Abstraction layer for smart contract build systems. |
| [foundry-rs](https://github.com/foundry-rs/foundry) | A framework for building, testing, fuzzing, and deploying Ethereum applications. |
| [medusa](https://github.com/crytic/medusa) | Parallelized, coverage-guided, mutational Solidity smart contract fuzzing, powered by go-ethereum. |
| [mythril](https://github.com/consensys/mythril) | Security analysis tool for EVM bytecode. Supports smart contracts built for Ethereum, Hedera, Quorum, Vechain, Rootstock, Tron and other EVM-compatible blockchains. |
| [n](https://github.com/tj/n) | A Node.js version manager. |
| [pyevmasm](https://github.com/crytic/pyevmasm) | A Python library for EVM (dis)assembly. |
| [rust](https://www.rust-lang.org/) | A systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. |
| [slither](https://github.com/crytic/slither) | Static analyzer for Solidity and Vyper |
| [solc-select](https://github.com/crytic/solc-select) | A Solidity compiler version manager. |
| [tealer](https://github.com/crytic/tealer) | Static analyzer for Teal. |
| [vyper](https://github.com/vyperlang/vyper) | Pythonic smart contract language for the EVM. |
