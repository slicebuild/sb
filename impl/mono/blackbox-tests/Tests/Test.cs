using System;
using System.Collections;
using System.Collections.Generic;

namespace sb_bbt.Tests
{
    public class Test
    {
        public const string Passed = "PASSED";
        public const string Failed = "FAILED";
        public IList<string> Output { get; } = new List<string>();

        public Test(App app)
        {
            App = app;
        }

        public virtual void Run()
        {
        }

        public App App { get; }

        public virtual void WriteStart()
        {
            Console.WriteLine();
            Console.WriteLine($"Started {GetType().Name}");
        }

        public virtual void WriteFinish(bool passed)
        {
            var s = passed ? Passed : Failed;
            Console.WriteLine($"Finished {GetType().Name} : {s}");
            if (passed)
                App.Passed.Add(GetType().Name);
            App.Failed.Add(GetType().Name);
        }
    }
}