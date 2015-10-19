using System;
using System.Collections.Generic;
using System.IO;
using System.IO.Compression;
using System.Linq;
using System.Net;
using sb.Utils;

namespace sb.App.Commands
{
    public class Fetch : ICommand
    {
        private readonly Args _args;

        public Fetch(Args args)
        {
            _args = args;
        }

        void ICommand.Run()
        {
            var latestBranch = FetchLatestBranchName();
            var tmpDir = Path.Combine(_args.EnvDir, Time.GetString());
            Directory.CreateDirectory(tmpDir);

            var zipUrl = string.Format(Const.ZipUrlFmt, latestBranch);
            var tmpFile = Path.Combine(tmpDir, "slices.zip");
            var url = _args.GetOption(Args.Options.Url, zipUrl);

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
                var directoryOld = Path.Combine(_args.SlicesDir, extractedDirName);

                if (Directory.Exists(directoryOld))
                    Directory.Delete(directoryOld, true);

                Directory.CreateDirectory(_args.SlicesDir);
                Directory.Move(directoryNew, directoryOld);
                Directory.Delete(tmpDir, true);
            }
        }

        private string FetchLatestBranchName()
        {
            var list = new List<SemVerName>();
            var branches = FetchBranches();
            foreach (var branch in branches.Where(b => b != "master"))
            {
                var svn = SemVerNameParser.Parse(branch);
                list.Add(svn);
            }
            list.Sort((x, y) => (y.NameVersion as IComparable).CompareTo(x.NameVersion));
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
