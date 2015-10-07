using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;

namespace cb.Utils
{
    public static class Fs
    {
        public static string ReplaceFileName(this string path, string newName)
        {
            return Path.Combine(Path.GetDirectoryName(path), newName);
        }

        public static void RunProcess(string name, string args, IList<string> stdout)
        {
            var processStartInfo = new ProcessStartInfo
            {
                CreateNoWindow = true,
                RedirectStandardOutput = true,
                RedirectStandardInput = true,
                UseShellExecute = false,
                FileName = name,
                Arguments = args
            };

            var process = new Process
            {
                StartInfo = processStartInfo,
                EnableRaisingEvents = true
            };

            process.OutputDataReceived += delegate (object sender, DataReceivedEventArgs e)
            {
                stdout.Add(e.Data);
                Console.WriteLine(e.Data);
            };

            process.Start();
            process.BeginOutputReadLine();
            process.WaitForExit();
            process.CancelOutputRead();
        }
    }
}