# Weekly Salary

## Write a function that takes a list of `hours` and returns the total weekly salary.

- The input list `hours` is listed sequentially, ordered from Monday to Sunday.
- A worker earns $10 an hour for the first 8 hours.
- For every overtime hour, he earns $15.
- On weekends, the employer pays double the usual rate, regardless how many hours were worked previously that week. For instance, 10 hours worked on a weekday would pay 80+30 = $110, but on a weekend it would pay 160+60 = $220.

### Examples
```js
weeklySalary([8, 8, 8, 8, 8, 0, 0]) ➞ 400
weeklySalary([10, 10, 10, 0, 8, 0, 0]) ➞ 410
weeklySalary([0, 0, 0, 0, 0, 12, 0]) ➞ 280
```

### Notes
- Every element in the array is greater than or equal to 0.
