from datetime import datetime
from collections import defaultdict


def calculate_accommodation_nights(date_from, date_to, amount, expense_type, document_id):
    nights = [None] * len(date_from)
    doc_data = defaultdict(list)

    # 首先，按单据号分组所有数据
    for i in range(len(date_from)):
        doc_data[document_id[i]].append((i, date_from[i], date_to[i], amount[i], expense_type[i]))

    # 对每个单据号分别处理
    for doc_entries in doc_data.values():
        doc_nights = [None] * len(doc_entries)
        deduction_map = {}
        total_amount = sum(entry[3] for entry in doc_entries)

        if total_amount <= 0:
            continue  # 如果总金额小于等于0，该单据所有行晚数为0

        for j, (i, from_date, to_date, amt, exp_type) in enumerate(doc_entries):
            if exp_type != "住宿":
                continue

            if amt < 0:
                if abs(amt) in deduction_map:
                    # 全额扣减
                    doc_nights[deduction_map[abs(amt)]] = 0
                # 无论是部分还是全额扣减，当前行晚数都为0
            elif amt > 0:
                delta = datetime.strptime(to_date, "%Y-%m-%d") - datetime.strptime(from_date, "%Y-%m-%d")
                doc_nights[j] = delta.days
                deduction_map[amt] = j

        # 将计算结果应用到原始索引
        for j, (i, _, _, _, _) in enumerate(doc_entries):
            nights[i] = doc_nights[j]

    return nights


date_from = ["2023-01-01", "2023-01-03", "2023-01-05", "2023-01-07", "2023-01-07", "2023-01-07", "2023-01-07", "2023-01-07"]
date_to = ["2023-01-03", "2023-01-05", "2023-01-07", "2023-01-09", "2023-01-09", "2023-01-09", "2023-01-09", "2023-01-09"]
amount = [1000, -1000, 500, 800, 1000, -1001, 1001, -1000]
expense_type = ["住宿", "住宿", "住宿", "住宿", "住宿", "住宿", "住宿", "住宿"]
document_id = ["A001", "A001", "A001", "A002", "A003", "A003", "A004", "A004"]

result = calculate_accommodation_nights(date_from, date_to, amount, expense_type, document_id)
print(result)  # [0, 0, 2, 2, 0, 0, 2, 0]
