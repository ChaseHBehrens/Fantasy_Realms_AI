# Fantasy Realms AI
This is an AI that plays the card game Fantasy Realms. 

## How to use
To play against the AI simply run the fantasy_realms_ui.exe file. This has a terminal interface that provides the ability to add bots and human players to a game. When adding bots you must input the name of the AI you wish to use, type either "Randy" for the random bot or "AutomatosV1" for the advanced bot. If only bots are added to the game the entire game will be simulated in the terminal displaying the results. If humans are added to the game you will need to use a physical copy of the game and relay information on each player's turn to keep the game state up to date. 

## How it works
I developed an API that enables the simulation of games and includes all rules of the game. It can be found [here](https://crates.io/crates/fantasy_realms_unofficial_api).
The UI crate has a terminal user interface that allows for simulated games with AI bots. 
The AI crate holds all of the logic for the AI bots. 
### Randy
Randy plays random moves. It has an equal probability of choosing each card from each card in the discard pile, or the deck. 
### AutomatosV1
AutomatosV1 plays the move that will give it the best possible score at the end of its turn. For each card checked it finds the maximum score of replacing every card in its hand. It checks each card in the discard pile. Then it checks every unknown card, and takes the average score. This is the expected evaluation for the deck. It then chooses the highest scoring option. After drawing a card it decides if it is beneficial to keep the card drawn or simply discard it.
### AutomatosV2
AutomatosV2 functions identically to AutomatosV1 with an added recursive depth search. The max depth is set to one move into the future so that it can be run in a reasonable amount of time.

## Performance
The function for scoring hands is relatively fast. It can run approximately 975 calculations per second. This is based on the time it took to compute the maximum scoring hand in the game. It took my computer 44 hours to score all 154,143,080 possible hands in the game. This confirms the maximum hand score of 397 points. 

### Randy
As expected the random bot did not perform well with hands scoring around 60 to 80 points on average. 
### AutomatosV1
Outperformed Human players. The ability to consider every possible outcome on its turn gave AutomatosV1 a considerable advantage. We played three four player games against the bot. It won one game, placed third, and placed second. Comparing its performance to our own it finished ahead of each of us in two games and we were each only able to beat it once. In addition to this I suspect that it was relatively unlucky with its hand draw in these games as simulating it against itself yielded higher scoring final hands. I did two four player simulations after these games containing three AutomatosV1 bots and one Randy bot. In each of these games the AI outscored our own high score of 281, scoring 295 and 325 points! 
![Screenshot 2025-06-07 220056](https://github.com/user-attachments/assets/87ac0031-7fe4-46d9-a774-3787a01e6c74)

### AutomatosV2
This was an attempt to improve upon the performance of AutomatosV1 by looking one move ahead. However, due to the large amount of randomness present in the game the extra depth of search did not have a noticeable effect on performance. Especially for the dramatic increase in compute time. 

## Documentation
Refer to the code files for more extensive documentation on how everything works. 
