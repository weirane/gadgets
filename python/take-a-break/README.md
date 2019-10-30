# take-a-break
A small pop up window to remind you to take a break. You can use it with cron. Pop up every hour:

```crontab
0 * * * * env DISPLAY=:1 /path/to/take-a-break.py <minutes>
```
