﻿using System;
using System.Collections.Generic;
using System.IO;
using System.Reflection;
using sb_bbt.Tests.Fetch;
using sb_bbt.Tests.Find;
using sb_bbt.Tests.Make;

namespace sb_bbt
{
    public class App
    {
        public string RootDir { get; }
        public string EnvDir { get; }
        public string SlicesDir { get; }

        public IList<string> Passed { get; } = new List<string>();
        public IList<string> Failed { get; } = new List<string>();

        public App()
        {
            RootDir = Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location);
            RootDir = RootDir ?? "";
            EnvDir = Path.Combine(RootDir, ".sb");
            SlicesDir = Path.Combine(EnvDir, "slices");
        }

        public void Run()
        {
            new TestFetch(this).Run();
            new TestFetchDeep(this).Run();
            new TestFind(this).Run();
            new TestFindRandom(this).Run();
            new TestMake(this).Run();

            Console.WriteLine();
            Console.WriteLine($"PASSED: {Passed.Count}");
            Console.WriteLine($"FAILED: {Failed.Count}");
            Console.WriteLine();
        }

    }
}