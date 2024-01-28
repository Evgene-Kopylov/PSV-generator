import random
from typing import List
from colorama import Fore, Style


class Deck:
    def __init__(self, suits: List[str] = None, nominal: List[str] = None):
        """
        Конструктор класса Deck.

        Параметры:
        - suits: список строк, представляющих масти карт.
        - nominal: список строк, представляющих номиналы карт.

        Атрибуты:
        - nominal: список строк, представляющих номиналы карт.
        - suits: список строк, представляющих масти карт.
        - deck: список карт в колоде.
        """
        self.nominal = nominal or ['T', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'ß', 'λ', '♛']
        self.suits = suits or ['♠', '♡', '◊', '♣']
        self.deck = None
        self.target = None
        self.target_chain = None
        self.line = None

    def new_deck(self):
        """
        Создает новую колоду карт, объединяя номиналы и масти,
        и исключая карты из целевой комбинации.

        Возвращает:
        - deck: список карт в колоде.
        """
        # Инициализация колоды без исключаемых карт
        self.deck = [f"{nominal}{suit}" for suit in self.suits for nominal in self.nominal]

        # Удаление карт из целевой комбинации
        for item in self.target:
            if isinstance(item, str):
                self.deck.remove(item)
            elif isinstance(item, list):
                for card in item:
                    self.deck.remove(card)

        return self.deck

    def shuffle_deck(self):
        """Перемешивает карты в колоде."""
        random.shuffle(self.deck)

    def take_cards(self, n):
        """
        Берет заданное количество карт из колоды.

        Параметры:
        - n: количество карт для взятия.

        Возвращает:
        - taken_cards: список взятых карт.
        """
        if n > len(self.deck):
            raise ValueError("Недостаточно карт в колоде")
        taken_cards = self.deck[:n]
        self.deck = self.deck[n:]  # Убирает взятые карты из колоды
        return taken_cards

    def replace_numbers(self, target: List):
        """
        Заменяет числа в списке на соответствующее количество карт.

        Параметры:
        - target: список, включающий числа и/или строки номиналов карт.

        Возвращает:
        - replaced: список, где числа заменены на соответствующее количество карт.
        """
        replaced = []
        for item in target:
            if isinstance(item, int):
                replaced += self.take_cards(item)
            else:
                replaced.append(item)
        return replaced

    def chain_check(self, chain: List) -> (bool, List):
        """
        Проверяет комбинацию карт по правилу схождения пасьянса.

        Параметры:
        - chain: список карт для проверки.
        - show: флаг для вывода промежуточных результатов.

        Возвращает:
        - True, если комбинация карт является пасьянсом, иначе False.
        """
        n = len(chain)
        report = ''
        for _ in range(n * n):
            for i in range(len(chain) - 2):
                if len(chain) == 2:
                    return chain
                if chain[i][:-1] == chain[i + 2][:-1] or chain[i][-1] == chain[i + 2][-1]:
                    pop = chain[i]
                    line = "\n" + "  ".join(chain[:i]) + ("  " if chain[:i] else '') \
                           + Fore.BLUE + chain[i] + Style.RESET_ALL \
                           + "  " + chain[(i + 1)] \
                           + Fore.BLUE + '  ' + chain[i + 2] + '  ' + Style.RESET_ALL \
                           + "  ".join(chain[(i + 3):])
                    line += "\n" + " " * (line.index(chain[i]) - 6) \
                            + Fore.WHITE + chain[i] + Style.RESET_ALL
                    chain.pop(i)
                    report += line
                    break
        if len(chain) == 2:
            print(report)
        return len(chain) == 2, chain

    def collect_target_chain(self, target: List[int | List[str]]) -> List[str]:
        """
        Собирает целевую комбинацию карт из заданного списка.

        Параметры:
        - target: список, представляющий целевую комбинацию карт.

        Возвращает:
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

        Параметры:
        - target: список, представляющий целевую комбинацию карт.

        Возвращает:
        - line: успешная комбинация карт.
        - iteration: номер попытки.
        - sequence: последовательность сложения.
        """
        self.target = target
        for iteration in range(1, 10001):
            self.new_deck()
            self.shuffle_deck()
            self.target_chain = self.collect_target_chain(target)
            self.line = "  ".join(self.target_chain)
            assert len(self.target_chain) > 2, "Длина целевой цепочки должна быть больше 2"
            folded, ending = self.chain_check(self.target_chain)
            if folded:
                line = f"""
-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
Целевая: {self.target}
Попытка: {iteration}
Комбинация: {self.line}
Остаток от сложения: {ending}
"""
                print(line)
                filename = "history.txt"
                with open(filename, "a", encoding="utf-8") as f:
                    f.write(line)
                return self.line, iteration, self.target_chain
        print("Не удалось найти успешную комбинацию после 10000 попыток.")
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
        # 'S',
        '♡',
    ])
    target = [5, ['2♡', '4☐'], 5]
    deck.psv(target=target)
