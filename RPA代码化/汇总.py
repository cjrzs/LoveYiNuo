from datetime import datetime
from collections import defaultdict


def calculate_accommodation_and_allowance(date_from, date_to, amount, expense_type, document_id):
    accommodation_nights = [None] * len(date_from)
    allowance_nights = [None] * len(date_from)
    accommodation_doc_data = defaultdict(list)
    allowance_doc_data = defaultdict(list)

    # 按单据号分组所有数据
    for i in range(len(date_from)):
        if expense_type[i] == "住宿":
            accommodation_doc_data[document_id[i]].append((i, date_from[i], date_to[i], amount[i], expense_type[i]))
        elif expense_type[i] == "差旅补助":
            allowance_doc_data[document_id[i]].append((i, date_from[i], date_to[i], amount[i], expense_type[i]))

    # 对每个单据号分别处理
    for doc_entries in accommodation_doc_data.values():
        doc_accommodation = [None] * len(doc_entries)
        deduction_map = defaultdict(list)
        minus_accommodation = set()
        for j, (i, from_date, to_date, amt, exp_type) in enumerate(doc_entries):
            if amt > 0:
                deduction_map[amt].append(j)
        total_amount = sum(entry[3] for entry in doc_entries)
        for j, (i, from_date, to_date, amt, exp_type) in enumerate(doc_entries):
            if j in minus_accommodation:
                continue
            if total_amount <= 0:
                doc_accommodation[j] = 0
                continue  # 如果总金额小于等于0，该单据所有行晚数为0
            if amt < 0:
                doc_accommodation[j] = 0
                if len(deduction_map[abs(amt)]) > 0:
                    # 全额扣减
                    t = deduction_map[abs(amt)].pop(0)
                    doc_accommodation[t] = 0
                    minus_accommodation.add(t)
                # 无论是部分还是全额扣减，当前行晚数都为0
            elif amt > 0:
                delta = datetime.strptime(to_date, "%Y-%m-%d %H:%M:%S") - datetime.strptime(from_date, "%Y-%m-%d %H:%M:%S")
                doc_accommodation[j] = delta.days
            else:
                # 金额为0 晚数一定为0
                doc_accommodation[j] = 0
        for j, (i, _, _, _, _) in enumerate(doc_entries):
            accommodation_nights[i] = doc_accommodation[j]

    for doc_entries in allowance_doc_data.values():
        doc_allowance = [None] * len(doc_entries)
        deduction_map = defaultdict(list)
        processed = set()
        minus_allowance = set()
        for j, (i, from_date, to_date, amt, exp_type) in enumerate(doc_entries):
            if amt > 0:
                deduction_map[amt].append(j)
        total_amount = sum(entry[3] for entry in doc_entries)
        for j, (i, from_date, to_date, amt, exp_type) in enumerate(doc_entries):
            if j in processed or j in minus_allowance:
                continue
            if total_amount <= 0:
                doc_allowance[j] = 0
                continue  # 如果总金额小于等于0，该单据所有行晚数为0
            if amt < 0:
                doc_allowance[j] = 0
                if len(deduction_map[abs(amt)]) > 0:
                    # 全额扣减
                    t = deduction_map[abs(amt)].pop(0)
                    doc_allowance[t] = 0
                    minus_allowance.add(t)
                # 无论是部分还是全额扣减，当前行晚数都为0
            elif amt > 0:
                # 检查是否有多行完全相同
                same_entries = [k for k, entry in enumerate(doc_entries) if
                                entry[1:5] == (from_date, to_date, amt, exp_type)]
                if len(same_entries) == len(doc_entries):
                    # 多行相同，只计算第一行
                    for k in same_entries[1:]:
                        doc_allowance[k] = 0
                        processed.add(k)
                delta = datetime.strptime(to_date, "%Y-%m-%d %H:%M:%S") - datetime.strptime(from_date, "%Y-%m-%d %H:%M:%S")
                doc_allowance[j] = delta.days + 1
            else:
                # 金额为0 晚数一定为0
                doc_allowance[j] = 0
        for j, (i, _, _, _, _) in enumerate(doc_entries):
            allowance_nights[i] = doc_allowance[j]

    return accommodation_nights, allowance_nights


# 示例使用
date_from = ["2023-01-01", "2023-01-01", "2023-01-01", "2023-01-01", "2023-01-01", "2023-01-01", "2023-01-10"]
date_to = ["2023-01-03", "2023-01-03", "2023-01-03", "2023-01-03", "2023-01-03", "2023-04-01", "2023-01-15"]
amount = [1000, 1000, 1000, 1000, 1000, 2000, 300]
expense_type = ["差旅补助", "差旅补助", "差旅补助", "差旅补助", "差旅补助", "住宿", "餐饮"]
document_id = ["A001", "A001", "A001", "A001", "A001", "A003", "A004"]

accommodation_result, allowance_result = calculate_accommodation_and_allowance(date_from, date_to, amount, expense_type,
                                                                               document_id)
print("住宿晚数:", accommodation_result)
print("补助天数:", allowance_result)

# 使用pandas展示结果
import pandas as pd

df = pd.DataFrame({
    '日期从': date_from,
    '日期到': date_to,
    '金额': amount,
    '费用类型': expense_type,
    '单据号': document_id,
    '住宿晚数': accommodation_result,
    '补助天数': allowance_result
})

print(df)
