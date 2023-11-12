import random
from typing import List, Optional


class Deck:
    def __init__(self, suits: List[str] = None, nominal: List[str] = None):
        """
        Конструктор класса Deck.

        Parameters:
        - values: список строк, представляющих значения карт.
        - nominal: список строк, представляющих номиналы карт.

        Attributes:
        - nominal: список строк, представляющих номиналы карт.
        - values: список строк, представляющих значения карт.
        - deck: список карт в колоде.
        """
        self.nominal = nominal or ['∆', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'ß', 'λ', '♛']
        self.suits = suits or ['☐', '○', '▲', 'Ω', '⌗']
        self.deck = None

    def new_deck(self):
        """
        Создает новую колоду карт, объединяя номиналы и значения.

        Returns:
        - deck: список карт в колоде.
        """
        self.deck = [f"{nominal}{suit}" for nominal in self.nominal for suit in self.suits]
        return self.deck

    def print_deck(self):
        """Выводит текущую колоду карт."""
        line = " ".join(self.deck)
        print(line)


if __name__ == '__main__':
    deck = Deck(suits=['♠', '♡', '◊', '♣'])
    deck.new_deck()
    deck.print_deck()
