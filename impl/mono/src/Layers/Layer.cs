using System.Collections.Generic;
using System.Linq;
using System.Text;
using cb.Slices;
using cb.Utils;

namespace cb.Layers
{
    public class Layer
    {
        private readonly Slice _slice;

        public Layer(Slice slice)
        {
            _slice = slice;
        }

        public string Name => _slice.Name;
        public SemName SemName => _slice.SemName;
        public SemVersion SemVersion => _slice.SemVersion;
        public IList<SliceSection> Sections => _slice.Sections;
        public LayerList Dependencies { get; } = new LayerList();
        public bool Written { get; set; }

        public void FindDependenciesRecursive(LayerList registryLayers)
        {
            foreach (var section in _slice.Sections.Where(s => s.SectionType == SliceSection.Type.DEP))
            {
                foreach (var line in section.Lines)
                {
                    var semName = new SemName(line);
                    var dep = registryLayers.FindLayer(semName.NamePart);
                    if (dep != null)
                    {
                        Dependencies.Add(dep);
                        dep.FindDependenciesRecursive(registryLayers);
                    }
                }
            }

            if (Dependencies.Count == 0)
            {
                foreach (var section in _slice.Sections.Where(s => s.SectionType == SliceSection.Type.OS))
                {
                    foreach (var line in section.Lines)
                    {
                        var semName = new SemName(line);
                        if (semName.NamePart != Name)
                        {
                            var os = registryLayers.FindLayer(semName.NamePart);
                            if (os != null)
                            {
                                Dependencies.Add(os);
                            }
                        }
                    }
                }
            }
        }

        public void Write(StringBuilder sb)
        {
            sb.AppendLine();
            foreach (var section in _slice.Sections.Where(s => s.SectionType != SliceSection.Type.OS && s.SectionType != SliceSection.Type.DEP))
            {
                sb.AppendLine();
                sb.AppendLine(section.SectionType.ToString());
                sb.AppendLine($"# {Name}-{SemVersion}");
                foreach (var line in section.Lines)
                {
                    sb.AppendLine(line);
                }
            }
            sb.AppendLine();
        }

        public override string ToString()
        {
            return _slice.SemName.ToString();
        }
    }
}