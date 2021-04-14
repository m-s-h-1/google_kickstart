    #include<bits/stdc++.h>
    using namespace std;
    #define LL long long
    #define pa pair<int,int>
    const int Maxn=610;
    const LL inf=1e18;
    const double pi=acos(-1.0);
    LL read()
    {
    	LL x=0,f=1;char ch=getchar();
    	while(ch<'0'||ch>'9'){if(ch=='-')f=-1;ch=getchar();}
    	while(ch>='0'&&ch<='9')x=x*10ll+ch-'0',ch=getchar();
    	return x*f;
    }
    int n,m,q,ans=0;
    struct Edge
    {
    	int u,v;LL w;
    }e[Maxn*Maxn];
    LL dis[Maxn][Maxn];
    vector<pa>h[Maxn];
    LL mn[Maxn][Maxn];
    int solve(int x,int y,int w)
    {
    	for(int i=1;i<=n;i++)
    	if(dis[i][x]+w+mn[i][y]<=0)return 1;
    	return 0;
    }
    int main()
    {
    	memset(mn,63,sizeof(mn));
    	n=read(),m=read();
    	for(int i=1;i<=n;i++)
    	for(int j=1;j<=n;j++)
    	dis[i][j]=inf;
    	for(int i=1;i<=m;i++)
    	{
    		int x=e[i].u=read(),y=e[i].v=read();
    		dis[x][y]=dis[y][x]=e[i].w=read();
    	}
    	for(int k=1;k<=n;k++)
    	for(int i=1;i<=n;i++)
    	for(int j=1;j<=n;j++)
    	dis[i][j]=min(dis[i][j],dis[i][k]+dis[k][j]);
    	for(int i=1;i<=n;i++)dis[i][i]=0;
    	q=read();
    	for(int i=1;i<=q;i++)
    	{
    		int u=read(),v=read(),l=read();
    		h[u].push_back(make_pair(v,l));
    		h[v].push_back(make_pair(u,l));
    	}
    	for(int i=1;i<=n;i++)
    	for(int j=1;j<=n;j++)
    	{
    		for(int k=0;k<h[i].size();k++)
    		{
    			int p=h[i][k].first;LL l=h[i][k].second;
    			mn[i][j]=min(mn[i][j],dis[p][j]-l);
    		}
    	}
    	for(int i=1;i<=m;i++)ans+=(solve(e[i].u,e[i].v,e[i].w)|solve(e[i].v,e[i].u,e[i].w));
    	printf("%d",ans);
    }