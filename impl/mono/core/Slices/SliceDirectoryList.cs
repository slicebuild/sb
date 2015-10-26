using System.Collections.Generic;
using System.IO;
using sb.Core.Utils;

namespace sb.Core.Slices
{
    public class SliceDirectoryList
    {
        private readonly List<SliceDirectory> _directories = new List<SliceDirectory>();

        public SliceDirectoryList(string root, int versionMajor)
        {
            
            var rootDir = new DirectoryInfo(root);
            
            foreach (var bunchDir in rootDir.EnumerateDirectories())
            {
                // Only add slices from the directories with same major version
                var svi = new SemVerInfo(bunchDir.Name);
                if (svi.NameMajor == versionMajor)
                {
                    _directories.Add(new SliceDirectory(rootDir, bunchDir, svi));
                }
            }
        }

        public SliceList Scan(SemVerInfo osInfo)
        {
            var list = new SliceList();
            foreach (var directory in _directories)
            {
                var slices = directory.Scan(osInfo);
                foreach (var slice in slices)
                {
                    list.Add(slice);
                }
            }
            return list;
        }    
    }
}