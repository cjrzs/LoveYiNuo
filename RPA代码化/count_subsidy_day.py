from datetime import datetime
from collections import defaultdict


def calculate_travel_allowance_nights(date_from, date_to, amount, expense_type, document_id):
    nights = [None] * len(date_from)
    doc_data = defaultdict(list)

    # 按单据号分组所有数据
    for i in range(len(date_from)):
        doc_data[document_id[i]].append((i, date_from[i], date_to[i], amount[i], expense_type[i]))

    # 对每个单据号分别处理
    for doc_entries in doc_data.values():
        doc_nights = [None] * len(doc_entries)
        deduction_map = {}
        processed = set()

        for j, (i, from_date, to_date, amt, exp_type) in enumerate(doc_entries):
            if exp_type != "差旅补助" or i in processed:
                continue

            if amt == 0:
                continue  # 金额为0，晚数为0

            if amt < 0:
                if abs(amt) in deduction_map:
                    # 全额扣减
                    deducted_index = deduction_map[abs(amt)]
                    doc_nights[deducted_index] = 0
                    processed.add(doc_entries[deducted_index][0])
                # 无论是部分还是全额扣减，当前行晚数都为0
            elif amt > 0:
                # 检查是否有多行完全相同
                same_entries = [k for k, entry in enumerate(doc_entries) if entry[1:4] == (from_date, to_date, amt)]
                if len(same_entries) > 1:
                    # 多行相同，只计算第一行
                    for k in same_entries[1:]:
                        processed.add(doc_entries[k][0])

                delta = (datetime.strptime(to_date, "%Y-%m-%d %H:%M:%S") - datetime.strptime(from_date, "%Y-%m-%d %H:%M:%S")).days
                if delta > 60:
                    doc_nights[j] = "异常"  # 日期差超过60天，标记为异常
                else:
                    doc_nights[j] = delta + 1  # 补助晚数 = 日期到 - 日期从 + 1
                deduction_map[amt] = j

        # 将计算结果应用到原始索引
        for j, (i, _, _, _, _) in enumerate(doc_entries):
            nights[i] = doc_nights[j]

    return nights


# 示例使用
date_from = ["2023-01-01", "2023-01-03", "2023-01-05", "2023-01-07", "2023-01-07", "2023-01-01"]
date_to = ["2023-01-03", "2023-01-05", "2023-01-07", "2023-01-09", "2023-01-09", "2023-04-01"]
amount = [1000, -1000, 500, 800, 800, 2000]
expense_type = ["差旅补助", "差旅补助", "差旅补助", "差旅补助", "差旅补助", "差旅补助"]
document_id = ["A001", "A001", "A001", "A002", "A002", "A003"]

result = calculate_travel_allowance_nights(date_from, date_to, amount, expense_type, document_id)
print(result)