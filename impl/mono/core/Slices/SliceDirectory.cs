using System.Collections.Generic;
using System.IO;
using sb.Core.Utils;

namespace sb.Core.Slices
{
    public class SliceDirectory
    {
        public SliceDirectory(string root, SemVerName semVerName)
        {
            Root = root;
            SemVerName = semVerName;
        }

        public string Root { get; }
        public SemVerName SemVerName { get; }

        public IList<Slice> FindByOs(string osName)
        {
            var list = new List<Slice>();
            ScanFiles(Root, list, osName);
            foreach (var dir in Directory.EnumerateDirectories(Root, "*.*", SearchOption.AllDirectories))
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

                var relPath = path.Replace(Root, "");
                var svn = SemVerNameParser.Parse(fileName, SemVerName);
                var lines = File.ReadAllLines(path);
                var slice = new Slice(relPath, svn, lines);

                if (slice.OsList.Contains(osName))
                    list.Add(slice);
            }
        }
    }
}