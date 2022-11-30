from os.path import dirname


def calculate_expense_report():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.readlines()
        for (index_1, line_1) in enumerate(lines):
            line_1 = int(line_1)
            for (index_2, line_2) in enumerate(lines):
                line_2 = int(line_2)
                if index_1 == index_2:
                    continue
                if line_1 + line_2 == 2020:
                    return line_1 * line_2


if __name__ == '__main__':
    print(calculate_expense_report())
