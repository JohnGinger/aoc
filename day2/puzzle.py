with open("input.txt") as input:
    sumMaxMin = 0
    sumEvenDivide = 0
    for line in input:
        nums = [int(s) for s in line.split()]
        sumMaxMin += (max(nums) - min(nums))
        for firstNum in range(len(nums)):
            for secondNum in range(firstNum + 1, len(nums)):
                if nums[firstNum] % nums[secondNum] == 0:
                    sumEvenDivide += int(nums[firstNum] / nums[secondNum])
                if nums[secondNum] % nums[firstNum] == 0:
                    sumEvenDivide += int(nums[secondNum] / nums[firstNum])
    print("Part 1 ", sumMaxMin)
    print("Part 2 ", sumEvenDivide)
