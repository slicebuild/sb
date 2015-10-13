using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;

namespace sb_bbt.Utils
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

        public static void DeleteFolder(string path)
        {
            Directory.Delete(path, true);
        }

        public static void DeleteFolderExcept(string path, string preservedPath)
        {
            foreach (var dir in Directory.GetDirectories(path).ToList())
            {
                if (dir.ToLower() == preservedPath.ToLower())
                    continue;
                Directory.Delete(dir, true);
            }
        }

        public static bool FolderExists(string path, string mask)
        {
            foreach (var dir in Directory.EnumerateDirectories(path))
            {
                if (new DirectoryInfo(dir).Name.Contains(mask))
                    return true;
            }
            return false;
        }

    }
}