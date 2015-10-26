using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Reflection;
using sb.Core.Utils;

namespace sb.Core.Slices
{
    public class SliceDirectoryList : List<SliceDirectory>
    {
        private List<SliceDirectory> _directories = new List<SliceDirectory>();

        public SliceDirectoryList(string root)
        {
            // Get the major part of the assembly version
            var fvi = FileVersionInfo.GetVersionInfo(Assembly.GetExecutingAssembly().Location);
            var currentMajorVersion = fvi.FileMajorPart;

            // Only add slices from the directories with same major version
            foreach (var dir in new DirectoryInfo(root).EnumerateDirectories())
            {
                var svi = new SemVerInfo(dir.Name);
                if (svi.NameMajor == currentMajorVersion)
                {
                    Add(new SliceDirectory(dir.Parent?.FullName, svi));
                    _directories.Add(new SliceDirectory(dir.Parent?.FullName, svi));
                }
            }
        }

        public SliceList Scan()
        {
            var list = new SliceList();
            foreach (var directory in _directories)
            {
                var slices = directory.FindByOs("debian");
                foreach (var slice in slices)
                {
                    list.Add(slice);
                }
            }
            return list;
        }    
    }
}