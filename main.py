import random
from typing import List, Optional, Union
import builtins


class Deck:
    def __init__(self, values: List[str], nominal: Optional[List[str]] = None):
        self.nominal = nominal or ['∆', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'σ', 'λ', '♛']
        self.values = values or ['☐', '○', '▲', 'Ω', '⌗']
        self.deck = None
        self.target_chain = None
        self.line = None

    def new_deck(self):
        self.deck = []
        for nominal in self.nominal:
            for value in self.values:
                self.deck.append(f"{nominal}{value}")
        return self.deck

    def print_deck(self):
        line = " ".join(self.deck)
        print(line)

    def shuffle_deck(self):
        random.shuffle(self.deck)

    def take(self, n):
        if n > len(self.deck):
            raise ValueError("Not enough cards in the deck")
        taken_cards = self.deck[:n]
        self.deck = self.deck[n:]  # Remove the taken cards from the deck
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

    def collect_target_chain(self, target: List[Union[List[str], int]]) -> List[str]:
        self.target_chain = []
        for item in target:
            match type(item):
                case builtins.int:
                    self.target_chain += self.take(item)
                case builtins.list:
                    self.target_chain += item
        return self.target_chain

    def psv(self, target: List[str]):
        for _ in range(10000):
            self.new_deck()
            self.shuffle_deck()
            self.target_chain = self.collect_target_chain(target)
            self.line = "  ".join(self.target_chain)
            assert len(self.target_chain) > 2, "Длинна целевой цепочки должна быть больше 2"
            if self.chain_check(self.target_chain):
                self.target_chain = self.line.split("  ")
                self.chain_check(self.target_chain, show=True)
                self.line = None
                return self.line


if __name__ == '__main__':
    deck = Deck(
        values=['△', '○', '☐', 'Ω'],
        nominal=['∆', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'σ', 'λ', '♛']
    )

    target = [3, ['4△'], 2, ['♛Ω', 'λ○'], 3]
    print(deck.psv(target=target))
