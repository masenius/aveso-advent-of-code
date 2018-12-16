namespace Day4
{
    public struct TimeStamp
    {
        public uint Year, Month, Day, Hour, Minute;

        // Crude minutes since year 0 function based on equal length years, months and so on
        public uint ToMinutes()
        {
            return this.Year * 525600 + this.Month * 43200 + this.Day * 1440 + this.Hour * 60 + this.Minute;
        }
    }

    public enum EventType
    {
        StartShift,
        FallAsleep,
        WakeUp
    }

    public struct GuardEvent
    {
        public TimeStamp Time;
        public EventType Type;
        public uint? Guard;

        public static int CompareByTimeStamps(GuardEvent e1, GuardEvent e2)
        {
            return e1.Time.ToMinutes().CompareTo(e2.Time.ToMinutes());
        }
    }
}
