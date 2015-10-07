using System;
using System.Collections.Generic;
using System.IO;
using cb.Utils;

namespace cb.Slices
{
    public class SliceDirectoryList : List<SliceDirectory>
    {
        public SliceDirectoryList(string root)
        {
            var listMaster = new List<Tuple<string, SemVersion, string>>();
            var listCustom = new List<Tuple<string, SemVersion, string>>();

            foreach (var dir in new DirectoryInfo(root).EnumerateDirectories())
            {
                var semName = new SemName(dir.Name);
                var semVersion = new SemVersion(semName.VersionPart, semName.PreReleasePart);
                if (semName.NamePart.EndsWith("custom"))
                    listCustom.Add(new Tuple<string, SemVersion, string>(semName.NamePart.ToLower(), semVersion,
                        dir.FullName));
                else
                    listMaster.Add(new Tuple<string, SemVersion, string>(semName.NamePart.ToLower(), semVersion,
                        dir.FullName));
            }

            listMaster.Sort((x, y) => y.Item2.CompareTo(x.Item2));
            listCustom.Sort((x, y) => y.Item2.CompareTo(x.Item2));

            if (listMaster.Count > 0)
                Add(new SliceDirectory(listMaster[0].Item3));
            if (listCustom.Count > 0)
                Add(new SliceDirectory(listCustom[0].Item3));
        }        
    }
}