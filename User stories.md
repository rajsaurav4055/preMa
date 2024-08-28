# PreMa - Decentralized prediction market for Cricket games

A decentralized platform where users can make predictions and place bets on ongoing and future cricket games. The games include all three cricket formats (Test, ODI, and T20) and games from all big leagues like IPL, Big Bash, etc.

# User Types

There will be two types of users on the platform:
1. Admin
2. Bettor

# User Stories for Bettors

**1. Registration/ Sign Up:** As a new bettor, I want to create my account by connecting to Solana wallets so that I can place bets.
- Click on the SignUp button
- A popup asks you to connect your wallet -> Select your preferred wallet -> Enter the password.

**2. Transfer SOL to platform wallet:** As a bettor, I want to transfer SOLs that I want to risk on bets to my platform wallet so that I can place bets.
- Click on the "My Wallet" button on the app.
- Click on "Add SOL" -> Confirm the amount of SOL you want to transfer -> Click Ok -> Confirmation popup displaying successful transfer.

**3. Navigation**
1. As a bettor, I want to see the list of active events/games I can place bets on. I should be able to filter the events based on leagues or formats.
2. As a bettor, I want to see the list of game cards to display the following:
   - Name of the league
   - Team A vs Team B
   - Question related to the game on which the user decides to place a bet
   - Two buttons with the probable answers to the question and the betting price for each answer. The user will place a bet on one of the answers to go in their favor in the future.
3. [Optional] As a bettor, I should be able to view the number of bets on a particular option, so that I can understand the user sentiment to make a correct decision.

**4. Placing Bets and after**
1. As a bettor, I want to place a bet on an active event/ game. After I place and the game is ongoing, I expect to see:
   - Increase or decrease in my earnings based on the win prediction and situation in the game.
2. As a bettor, there are two ways to exit the bet:
   - I want to be able to exit my trades at any moment and the difference is added to my platform wallet (if I win).
   - or, once the game ends my bet ends automatically and I am rewarded if I win.

# User stories for Admin
1. As an admin, I should be able to CRUD bids, and assign prices to each betting option.
2. As an admin, I should be able to manage users. 


