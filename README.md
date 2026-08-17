# Analog Seed Entropy

Generate truly random [BIP39 seed words](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) using only dice or playing cards.
Never trust a single wallet vendor to generate sufficient entropy.

## How it works
- [Roll dice](#dice-rolls) or [deal playing cards](#playing-cards)
- Consult the lookup table to find your seed word
- Repeat until you have 11 or 23 seed words
- Import into your wallet and choose the last word

Advanced users can verify the address derivation is correct to keep hardware vendors honest

## Playing Cards

### Instructions
1. Create a deck of 14 cards by taking all spade cards (♠) and the ace-of-hearts (A♥)
2. Perform a [wash shuffle](https://en.wikipedia.org/wiki/Shuffling#Corgi) by mixing the cards thoroughly face-down on a table
3. Square up the deck and deal out 3 cards
4. Consult the [card lookup table](https://seed-cat.github.io/analog-seed-entropy/cards.html) to find your seed word
5. If your 1st card is A♥ and you can't find your cards in the lookup table, reshuffle **ALL the cards** and try again
6. Repeat until you have enough seed words

### Rationale
- Most people will have trouble randomizing a 52-card deck, but should be able to randomize 14 cards easily
- Dealing 3 cards results in `14 × 13 × 12 = 2184` possible outcomes, so we must retry `136 = 2184 - 2048` outcomes or roughly `1 / 16` attempts
- To achieve this, we allow the A♥ to be the first card in only 20 outcomes so users can quickly identify when they must retry
- Retrying is a form of [rejection sampling](https://en.wikipedia.org/wiki/Rejection_sampling) that eliminates any bias so long as users reshuffle all cards

### Specification
- Let `i ∈ [0, 13]` where the ace-of-hearts is `i = 0` followed by the 13 spades in-order (ace-of-spades `i = 1`, 2-of-spades `i = 2`, ..., king-of-spades `i = 13`)
- The user deals a random tuple `(i₀, i₁, i₂)` without repetition
- If `i₀ = 0` and `i₁ > 2`, skip this tuple
- If `i₀ = 0` and `i₁ = 2` and `i₂ > 9`, skip this tuple
- Order the remaining `2048` tuples to map the tuples onto the seed word list

## Dice Rolls

### Instructions
1. Roll two dice, rerolling if you get doubles below 5 (reroll ⚀⚀, ⚁⚁, ⚂⚂, ⚃⚃)
2. Repeat the previous step, so that you have total of 4 dice rolls
3. Finally, roll a single die, giving you a 5th dice roll which will either be low (⚀⚁⚂) or high (⚃⚄⚅)
4. Consult the [dice lookup table](https://seed-cat.github.io/analog-seed-entropy/dice.html) to find your seed word
5. Repeat until you have enough seed words

### Rationale
- Rolling 2 dice results in `6 × 6 = 36` possible outcomes
- Retrying `4` of the outcomes (using doubles for easy identification) results in `32` unbiased outcomes through [rejection sampling](https://en.wikipedia.org/wiki/Rejection_sampling)
- Performing this twice, we get `32 × 32 = 1024` possible outcomes
- The final dice roll can be low or high, resulting in `2 × 1024 = 2048` outcomes

### Specification
- Let `i ∈ [0, 5]` represent a die roll
- The user rolls a tuple of dice rolls `(i₀, i₁, i₂, i₃, i₄)`
- If `i₀ = i₁` and `i₀ < 4`, skip this tuple
- If `i₂ = i₃` and `i₂ < 4`, skip this tuple
- If `i₄ ∈ [0, 2]` let `i₄ = 0`, else if `i₄ ∈ [3, 5]` let `i₄ = 1`
- Order the remaining `2048` tuples to map the tuples onto the seed word list