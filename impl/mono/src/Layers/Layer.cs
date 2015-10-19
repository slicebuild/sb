using System.Collections.Generic;
using System.Linq;
using System.Text;
using sb.Slices;
using sb.Utils;

namespace sb.Layers
{
    public class Layer
    {
        private readonly Slice _slice;

        public Layer(LayerList registryLayers, Slice slice)
        {
            RegistryLayers = registryLayers;
            _slice = slice;
        }

        public LayerList RegistryLayers { get; }
        public string RelPath => _slice.RelPath;
        public SemVerName SemVerName => _slice.SemVerName;
        public IList<SliceSection> Sections => _slice.Sections;
        public IList<Layer> Dependencies { get; private set; } = new List<Layer>();
        public bool Written { get; set; }

        public virtual void FindDependenciesRecursive(LayerList registryLayers)
        {
            var depNames = new List<string>();
            var osNames = new List<string>();

            foreach (var section in _slice.Sections.Where(s => s.SectionType == SliceSection.Type.DEP))
            {
                foreach (var line in section.Lines)
                {
                    var svn = SemVerNameParser.Parse(line);
                    if (!depNames.Contains(svn.Name))
                    {
                        depNames.Add(svn.Name);
                    }
                }
            }

            foreach (var section in _slice.Sections.Where(s => s.SectionType == SliceSection.Type.OS))
            {
                foreach (var line in section.Lines)
                {
                    var svn = SemVerNameParser.Parse(line);
                    if (registryLayers.OsName == svn.Name && svn.Name != SemVerName.Name && !osNames.Contains(svn.Name))
                    {
                        osNames.Add(svn.Name);
                    }
                }
            }

            Dependencies = registryLayers.FindLayers(depNames.ToArray());
            foreach (var dep in Dependencies)
            {
                dep.FindDependenciesRecursive(registryLayers);
            }
            
            var oses = registryLayers.FindLayers(osNames.ToArray());
            foreach (var os in oses)
            {                
                Dependencies.Add(os);
            }
        }

        public virtual void Write(StringBuilder sb)
        {
            sb.AppendLine();
            foreach (var section in _slice.Sections.Where(s => s.SectionType != SliceSection.Type.OS && s.SectionType != SliceSection.Type.DEP))
            {
                sb.AppendLine();
                sb.AppendLine(section.SectionType.ToString());
                sb.AppendLine($"# {SemVerName}");
                foreach (var line in section.Lines)
                {
                    sb.AppendLine(line);
                }
            }
            sb.AppendLine();
        }

        public override bool Equals(object obj)
        {
            var other = (Layer) obj;
            return SemVerName.Equals(other.SemVerName);
        }

        public override int GetHashCode()
        {
            return SemVerName.GetHashCode();
        }

        public override string ToString()
        {
            return _slice.ToString();
        }
    }
}