using System.Collections.Generic;
using System.IO;
using sb.Core.Utils;

namespace sb.Core.Slices
{
    public class SliceDirectory
    {
        public SliceDirectory(string rootPath, SemVerInfo semVerInfo)
        {
            RootPath = rootPath;
            SemVerInfo = semVerInfo;
        }

        public string RootPath { get; }
        public SemVerInfo SemVerInfo { get; }

        public IList<Slice> FindByOs(string osName)
        {
            var list = new List<Slice>();
            ScanFiles(RootPath, list, osName);
            foreach (var dir in Directory.EnumerateDirectories(RootPath, "*.*", SearchOption.AllDirectories))
            {
                ScanFiles(dir, list, osName);
            }            
            return list;
        }

        private void ScanFiles(string dir, IList<Slice> list, string osName)
        {
            foreach (var path in Directory.EnumerateFiles(dir))
            {
                var fileName = new FileInfo(path).Name;
                if (fileName.StartsWith("."))
                    continue;

                var ext = Path.GetExtension(fileName);
                if (ext == ".md" || ext == ".txt")
                    continue;

                var relPath = path.Replace(RootPath, "");
                var svi = new SemVerInfo(SemVerInfo.NameSemVer, fileName);
                var lines = File.ReadAllLines(path);
                var slice = new Slice(relPath, svi, lines);

                if (slice.OsList.Contains(osName))
                    list.Add(slice);
            }
        }
    }
}