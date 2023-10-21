import random
from typing import List, Optional


class Deck:
    def __init__(self, values: List[str], nominal: Optional[List[str]] = None):
        self.nominal = nominal or ['∆', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'σ', 'λ', '♛']
        self.values = values or ['☐', '○', '▲', 'Ω', '⌗']
        self.chain = None
        self.target_chain = None
        self.line = None

    def new_deck(self):
        self.chain = []
        for nominal in self.nominal:
            for value in self.values:
                self.chain.append(f"{nominal}{value}")
        return self.chain

    def print_deck(self):
        line = " ".join(self.chain)
        print(line)

    def shuffle_deck(self):
        random.shuffle(self.chain)

    def take(self, n):
        if n > len(self.chain):
            raise ValueError("Not enough cards in the deck")
        taken_cards = self.chain[:n]
        self.chain = self.chain[n:]  # Remove the taken cards from the deck
        return taken_cards

    def chain_check(self, chain: List, show: bool = False):
        n = len(chain)
        if show:
            print(self.line)
            print(chain)
        for _ in range(n * n):
            for i in range(len(chain) - 2):
                if len(chain) == 2:
                    return chain
                if chain[i][:-1] == chain[i + 2][:-1] or chain[i][-1] == chain[i + 2][-1]:
                    pop = chain[i]
                    chain.pop(i)
                    if show: print(chain, pop)
                    break
        return len(chain) == 2

    def psv(self, target: List[str], before: int = 0, after: int = 0):
        for _ in range(10000):
            self.new_deck()
            self.shuffle_deck()
            self.target_chain = self.take(before) + target + self.take(after)
            self.line = "  ".join(self.target_chain)
            assert len(self.target_chain) > 2, "Длинна целевой цепочки должна быть больше 2"
            if self.chain_check(self.target_chain):
                self.target_chain = self.line.split("  ")
                self.chain_check(self.target_chain, show=True)
                self.line = None
                return self.line


if __name__ == '__main__':
    # deck = Deck(['▲', 'Ω', '丄'])
    # chain.new_deck()  # Create the deck
    # chain.print_deck()
    # chain.shuffle_deck()  # Shuffle the deck
    # chain.print_deck()  # Print the shuffled deck
    # chain = Chain(['▲', 'Ω', '丄'])
    # chain.new_deck()  # Create and initialize the deck
    # chain.shuffle_deck()  # Shuffle the deck
    #
    # # n = 5  # Number of cards to take
    # # taken_cards = chain.take(n)
    # # print(f"Taken cards: {taken_cards}")

    # target = ['λ○', '8○']  # "Пап, дай денег"
    # print(chain.psv(target=target, before=2, after=5))
    # print(chain.target_chain)

    # deck = Deck(['○', '♡', '⌗', 'Ω', '丄'])
    #
    # target = ['λ○', '3♡', 'σ⌗', '7♡', '4⌗']
    # print(deck.psv(target=target, before=4, after=3))
    # print(deck.target_chain)

    # deck = Deck(['♡', '▲', '♧', '○'])
    #
    # target = ['8○', '6♡', '9♠', '4♧', '10○']
    # print(deck.psv(target=target, before=3, after=4))
    # print(deck.target_chain)

    # deck = Deck(['▲', '○', '♧'])
    #
    # target = ['Δ▲', '8○', '4○', '8♧']
    # print(deck.psv(target=target, before=3, after=2))
    # print(deck.target_chain)

    deck = Deck(['△', '○', '☐', 'Ω'])
    
    target = ['4△', '♛Ω', 'λ○']
    print(deck.psv(target=target, before=3, after=2))
    print(deck.target_chain)

    print('💀')