using HxPosed.Plugins.Permissions;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace HxPosed.Plugins.Config
{

    public class PluginConfig
    {
        public required int Revision { get; set; } = 0;
        public required Guid? Guid { get; set; }
        public required string Name { get; set; } = "Name";
        public required string Description { get; set; } = "Description";
        public required uint Version { get; set; }
        public required string Url { get; init; } = "Url";
        public required string Author { get; init; } = "Author";
        public required string Icon { get; init; } = "App24";
        public required string Path { get; init; } = "Path";
        [JsonConverter(typeof(JsonStringEnumConverter))]
        public required PluginPermissions Permissions { get; set; } = PluginPermissions.None;

        public required List<ConfigDownload> Downloads { get; set; } = [];
    }
}
