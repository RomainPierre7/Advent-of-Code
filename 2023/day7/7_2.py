import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '7_input.txt')
input_file = open(file_path, 'r')

hands = []
bids = []

for line in input_file:
    hand, bid = line.split()
    hands.append(hand)
    bids.append(int(bid))

FIVE_KIND = 6
FOUR_KIND = 5
FULL_HOUSE = 4
THREE_KIND = 3 
TWO_PAIR = 2
ONE_PAIR = 1
HIGHT_CARD = 0

def hand_type(hand):
    scores = []
    count = 0
    pair = False
    pair_value = None
    joker_used = False
    joker_used_to_pair = False
    double_pair = False
    for card in hand:
        for i in range(len(hand)):
            if hand[i] == card or hand[i] == 'J':
                if hand[i] == 'J':
                    joker_used = True
                count += 1
            if count == 2:
                if not pair:
                    pair = True
                    pair_value = card
                    if joker_used:
                        joker_used_to_pair = True
                if card != pair_value and not (joker_used and joker_used_to_pair):
                    double_pair = True
        if count == 5:
            scores.append(FIVE_KIND)
        if count == 4:
            scores.append(FOUR_KIND)
        if count == 3:
            full_house_count = 0
            for other_card in hand:
                if other_card != card and not (other_card == 'J' and joker_used):
                    for j in range(len(hand)):
                        if hand[j] == other_card or (hand[j] == 'J' and not joker_used):
                            full_house_count += 1
                    if full_house_count == 2:
                        scores.append(FULL_HOUSE)
                    full_house_count = 0
            scores.append(THREE_KIND)
        count = 0
        joker_used = False
    if double_pair:
        scores.append(TWO_PAIR)
    if pair:
        scores.append(ONE_PAIR)
    scores.append(HIGHT_CARD)
    return max(scores)

def card_value(card):
    match card:
        case 'A':
            return 14
        case 'K':
            return 13
        case 'Q':
            return 12
        case 'J':
            return 1
        case 'T':
            return 10
        case _:
            return int(card)

# Return 1 if hand1 wins, 2 if hand2 wins, 0 if hand1 == hand2
def winning_hand(hand1, hand2):
    if (hand_type(hand1) > hand_type(hand2)):
        return 1
    if (hand_type(hand1) < hand_type(hand2)):
        return 2
    for i in range(len(hand1)):
        if card_value(hand1[i]) > card_value(hand2[i]):
            return 1
        if card_value(hand1[i]) < card_value(hand2[i]):
            return 2
    return 0

for i in range(len(hands) - 1):
    for j in range(1, len(hands) - i):
        if winning_hand(hands[j-1], hands[j]) == 1:
            hands[j-1], hands[j] = hands[j], hands[j-1]
            bids[j-1], bids[j] = bids[j], bids[j-1]

result = 0
for i in range(len(hands)):
    result += bids[i] * (i+1)

print(result)