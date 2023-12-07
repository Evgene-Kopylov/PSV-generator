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
        self.nominal = nominal or ['T', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'ß', 'λ', '♛']
        self.suits = suits or ['♠', '♡', '◊', '♣']
        self.deck = None
        self.target_chain = None
        self.line = None

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

    def chain_check(self, chain: List, show: bool = False):
        """
        Проверяет комбинацию карт по правилу схождения пасьянса.

        Parameters:
        - chain: список карт для проверки.
        - show: флаг для вывода промежуточных результатов.

        Returns:
        - True, если комбинация карт является пасьянсом, иначе False.
        """
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
                    if show:
                        print(chain, pop)
                    break
        return len(chain) == 2, chain

    def collect_target_chain(self, target: List[int | List[str]]) -> List[str]:
        """
        Собирает целевую комбинацию карт из заданного списка.

        Parameters:
        - target: список, представляющий целевую комбинацию карт.

        Returns:
        - target_chain: целевая комбинация карт.
        """
        self.target_chain = []
        for item in target:
            if isinstance(item, int):
                self.target_chain += self.take_cards(item)
            elif isinstance(item, list):
                self.target_chain += item
        return self.target_chain

    def psv(self, target: List[str]):
        """
        Выполняет пасьянс, пока не получится, и выводит успешную комбинацию, номер попытки и последовательность сложения.

        Parameters:
        - target: список, представляющий целевую комбинацию карт.

        Returns:
        - line: успешная комбинация карт.
        - iteration: номер попытки.
        - sequence: последовательность сложения.
        """
        for iteration in range(1, 10001):
            self.new_deck()
            self.shuffle_deck()
            self.target_chain = self.collect_target_chain(target)
            self.line = "  ".join(self.target_chain)
            assert len(self.target_chain) > 2, "Длина целевой цепочки должна быть больше 2"
            folded, ending = self.chain_check(self.target_chain)
            if folded:
                print(f"Попытка: {iteration}")
                print(f"Комбинация: {self.line}")
                print(f'Остаток от сложения: {ending}')
                return self.line, iteration, self.target_chain
        print("No successful combination found after 10000 iterations.")
        return None

    def print_deck(self):
        """Выводит текущую колоду карт."""
        line = " ".join(self.deck)
        print(line)


if __name__ == '__main__':
    deck = Deck(suits=[
        '☐',
        'L',
        '▲',
        'Ω',
        # 'S'
    ])
    target = [4, ['λL'], 2, ['T▲'], 3]
    deck.psv(target=target)
