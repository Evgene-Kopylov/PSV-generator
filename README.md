# PSV-generator
 Генерация цепочек для ПСВ

# Запуск
Через main.py

указать целевую цепочку
```python

target = [
    3,    # число случайные карты
    '4☐', # фиксированная карта в фиксинованной позиции
    2, ...]

```

Номиналы и масти можно задавать аргументамию. Иначе будут использованы значения по-умолчанию.

```python

deck = Deck(suits=[
    '☐',
    '▲',
    '○',
    ],
    nominal=[....]
)



```



## Значения по умолчанию.

```python
self.nominal = nominal or ['T', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'ß', 'λ', '♛']
self.suits = suits or ['♠', '♡', '◊', '♣']
```

# История
Расклады логируются в [history.txt](history.txt) 
