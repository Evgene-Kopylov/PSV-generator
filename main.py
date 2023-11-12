import random
from typing import List, Optional


class Deck:
    def __init__(self, suits: List[str] = None, nominal: List[str] = None):
        """
        Конструктор класса Deck.

        Parameters:
        - suits: список строк, представляющих масти карт.
        - nominal: список строк, представляющих номиналы карт.

        Attributes:
        - nominal: список строк, представляющих номиналы карт.
        - suits: список строк, представляющих масти карт.
        - deck: список карт в колоде.
        """
        self.nominal = nominal or ['∆', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'ß', 'λ', '♛']
        self.suits = suits or ['☐', '○', '▲', 'Ω', '⌗']
        self.deck = None

    def new_deck(self):
        """
        Создает новую колоду карт, объединяя номиналы и масти.

        Returns:
        - deck: список карт в колоде.
        """
        self.deck = [f"{nominal}{suit}" for suit in self.suits for nominal in self.nominal]
        return self.deck

    def shuffle_deck(self):
        """Перемешивает карты в колоде."""
        random.shuffle(self.deck)

    def take_cards(self, n):
        """
        Берет заданное количество карт из колоды.

        Parameters:
        - n: количество карт для взятия.

        Returns:
        - taken_cards: список взятых карт.
        """
        if n > len(self.deck):
            raise ValueError("Not enough cards in the deck")
        taken_cards = self.deck[:n]
        self.deck = self.deck[n:]  # Убирает взятые карты из колоды
        return taken_cards

    def replace_numbers(self, target: List):
        """
        Заменяет числа в списке на соответствующее количество карт.

        Parameters:
        - target: список, включающий числа и/или строки номиналов карт.

        Returns:
        - replaced: список, где числа заменены на соответствующее количество карт.
        """
        replaced = []
        for item in target:
            if isinstance(item, int):
                replaced += self.take_cards(item)
            else:
                replaced.append(item)
        return replaced

    def print_deck(self):
        """Выводит текущую колоду карт."""
        line = " ".join(self.deck)
        print(line)


if __name__ == '__main__':
    deck = Deck(suits=['♠', '♡', '◊', '♣'])
    deck.new_deck()
    deck.shuffle_deck()
    deck.print_deck()

    target_list = [3, '10◊', 2]
    replaced_list = deck.replace_numbers(target_list)
    print(f"\nReplaced list: {replaced_list}")
    deck.print_deck()