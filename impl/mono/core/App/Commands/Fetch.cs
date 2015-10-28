using System;
using System.Collections.Generic;
using System.IO;
using System.IO.Compression;
using System.Linq;
using System.Net;
using sb.Core.Utils;

namespace sb.Core.App.Commands
{
    public class Fetch : ICommand
    {
        public Fetch(Args args)
        {
            Args = args;
        }

        public Args Args { get; }

        void ICommand.Run()
        {
            var latestBranch = FetchLatestBranchName();
            var tmpDir = Path.Combine(Args.EnvDir, Time.GetString());
            Directory.CreateDirectory(tmpDir);

            var zipUrl = string.Format(Const.ZipUrlFmt, latestBranch);
            var tmpFile = Path.Combine(tmpDir, "slices.zip");
            var url = Args.GetOption(Args.Options.Url, zipUrl);

            using (var wc = new WebClient())
            {
                wc.DownloadFile(url, tmpFile);
            }

            ZipFile.ExtractToDirectory(tmpFile, tmpDir);
            var extractedDir = Directory.GetDirectories(tmpDir).FirstOrDefault();

            if (extractedDir != null)
            {
                var extractedDirName = new DirectoryInfo(extractedDir).Name;

                var directoryNew = Path.Combine(tmpDir, extractedDirName);
                var directoryOld = Path.Combine(Args.SlicesDir, extractedDirName);

                if (Directory.Exists(directoryOld))
                    Directory.Delete(directoryOld, true);

                Directory.CreateDirectory(Args.SlicesDir);
                Directory.Move(directoryNew, directoryOld);
                Directory.Delete(tmpDir, true);
            }
        }

        /// <summary>
        /// Fetches the latest version with the same MAJOR part as the executable
        /// </summary>
        /// <returns></returns>
        private string FetchLatestBranchName()
        {
            var list = new List<SemVerInfo>();
            var branches = FetchBranches();
            foreach (var branch in branches.Where(b => b != "master"))
            {
                var svi = new SemVerInfo(branch);
                if (svi.NameMajor == Args.VersionInfo.FileMajorPart)
                    list.Add(svi);
            }
            list.Sort((x, y) => y.CompareTo(x));
            return list[0].ToString();
        }

        private IEnumerable<string> FetchBranches()
        {
            string json;
            using (var wc = new WebClient())
            {
                wc.Headers.Add("user-agent", "Mozilla/4.0 (compatible)");
                wc.Headers.Add("keep-alive", "false");
                json = wc.DownloadString("https://api.github.com/repos/slicebuild/slices/branches");
            }

            var items = json.Replace("\"", "").Split(new[] {'{', '}', ',', '[', ']'}, StringSplitOptions.RemoveEmptyEntries).ToList();
            items = items.Where(l => l.StartsWith("name:")).ToList();
            for (var i = 0; i < items.Count; i++)
            {
                items[i] = items[i].Replace("name:", "");
            }
            
            return items;
        }
    }
}
