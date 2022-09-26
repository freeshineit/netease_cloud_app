export interface MenuItem {
  title: string;
  /** history link */
  link?: string;
  /** a link */
  href?: string;
}

export const HEADER_MENU: MenuItem[] = [
  {
    title: "个性推荐",
    link: "/?type=recommend"
  },
  {
    title: "歌单",
    link: "/?type=song_sheet"
  },
  {
    title: "排行榜",
    link: "/?type=ranking"
  },
  {
    title: "歌手",
    link: "/?type=singer"
  },
  {
    title: "最新音乐",
    link: "/?type=music"
  }
];

export const SIDER_MENU1: MenuItem[] = [
  {
    title: "发现音乐",
    link: "/"
  },
  {
    title: "播客",
    link: "/podcast"
  },
  {
    title: "私人FM",
    link: "/fm"
  },
  {
    title: "视频",
    link: "/videos"
  },
  {
    title: "关注",
    link: "/follow"
  }
];
export const SIDER_MENU2: MenuItem[] = [
  {
    title: "我喜欢的音乐",
    link: "/favorite"
  },
  {
    title: "iTunes音乐",
    link: "/itunes"
  },
  {
    title: "下载管理",
    link: "/download"
  },
  {
    title: "最近播放",
    link: "/recent_plays"
  },
  {
    title: "我的音乐云盘",
    link: "/cloud"
  },
  {
    title: "我的播客",
    link: "/my_podcast"
  },
  {
    title: "我的收藏",
    link: "/my_collection"
  }
];
export const SIDER_MENU3: MenuItem[] = [
  {
    title: "我喜欢的音乐",
    link: "/"
  }
];
