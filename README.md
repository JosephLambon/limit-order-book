# Simulated Limit Order Book

## Architecture

### Engine
- One thread per Instrument. Uses message passing via **channels**
- Within each thread:
    - Listens for **commands**
    - **Emits events**, driving the order book to match/execute trades

### Order Book
- **Listens for events** emitted by Engine
- **Handles** ask/buy sides
- **Matches** orders
- **Executes** trades