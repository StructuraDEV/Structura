# Structura Architecture

## 1. Overview

Structura fuses **AI-driven resource optimization** with **Solana’s** low-cost, high-throughput blockchain. This document outlines the **system architecture**, explaining:

- **On-chain** elements (smart contracts, project accounts, milestone tracking).  
- **Off-chain** components (AI modules, data oracles, resource models).  
- **Data flow** between these components and **user interactions**.

## 2. Components

### 2.1 On-Chain (Solana)

1. **Structura Program (Anchor-based)**  
   - Manages **ProjectAccounts**, storing details like budgets, deadlines, and completed milestones.  
   - Tracks **progress** updates and can trigger **reward distributions** (e.g., $STURA tokens, once integrated).  
   - Ensures **immutability** and **accountability**—all updates require authorized signatures.

2. **$STURA Token** (SPL Token)  
   - A Solana Program Library (SPL) token that represents the platform’s utility and reward currency.  
   - (Note: Actual token creation may happen via Pump.fun, but it integrates into the on-chain program for rewards.)

### 2.2 Off-Chain (AI & Data Feeds)

1. **AI Engine / Resource Optimization**  
   - An external service (e.g., Python/Node) that analyzes project data (budgets, schedules, resource usage).  
   - Uses **historical** and **real-time** data to forecast delays, budget overruns, or material shortages.  
   - Communicates **recommendations** back to the **on-chain** side or directly to the user interface.

2. **Data Oracles**  
   - If the AI or external data sources need to **publish** results on-chain, an oracle (e.g., Switchboard, Pyth) can be used.  
   - Converts **off-chain** signals (AI predictions, IoT sensor data) into **on-chain** instructions.

3. **Project Collaboration Tools**  
   - A web or mobile app that allows contractors, architects, and stakeholders to **view AI insights** and **submit updates** (stored on-chain).

## 3. Data Flow

1. **Project Initialization**  
   - A user calls `initialize_project` on the **Structura Program**, creating a `ProjectAccount`.  
   - Basic info—project name, budget, deadline—gets stored on-chain.

2. **Off-Chain AI Analysis**  
   - AI engine pulls data (budgets, schedules) from on-chain or an off-chain database.  
   - Runs predictive models to forecast potential issues (delays, cost overruns).

3. **AI Recommendations → On-Chain**  
   - If needed on-chain, an **oracle** or a **trusted signer** writes AI summary data to `ProjectAccount`.  
   - Otherwise, the AI findings remain off-chain (displayed in the user dashboard).

4. **Milestone Tracking & Rewards**  
   - Construction teams submit milestone updates via `update_milestone`.  
   - The program increments `milestones_completed` in `ProjectAccount`.  
   - A `distribute_rewards` call (optional) could send $STURA tokens to contributors upon milestone completion.

5. **Escrow & Accountability** (Optional Enhancement)  
   - Funds can be escrowed until all parties confirm milestone completion.  
   - Minimizes disputes by providing a verifiable blockchain record.

## 4. Smart Contract Architecture

- **InitializeProject**  
  - **Input**: name, budget  
  - **Output**: creates a `ProjectAccount` with initial milestones set to 0  

- **UpdateMilestone**  
  - **Input**: milestone index, completed bool  
  - **Output**: increments `milestones_completed` in `ProjectAccount` if completed  

- **DistributeRewards**  
  - **Input**: amount (in $STURA)  
  - **Output**: placeholder or real SPL token transfer to participants  

