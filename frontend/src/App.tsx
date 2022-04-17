import { useAuth } from "./AuthenticationContext";
import { Route, Routes, useSearchParams, useNavigate } from "react-router-dom";

const App = () => {
  return (
    <Routes>
      <Route path="/test/123" element={<Test />} />
      <Route path="/auth" element={<Auth />} />
      <Route path="/*" element={<Home />} />
    </Routes>
  );
};

const Home = () => {
  const authContext = useAuth();

  return (
    <div>
      <h1>Home!</h1>
      <h3>{authContext.client_id}</h3>
      <Login />
    </div>
  );
};

const Test = () => {
  return (
    <div>
      <h1>this is a test page!, get me back here!</h1>
      <Login />
    </div>
  );
};

const Auth = () => {
  const authContext = useAuth();
  let [searchParams] = useSearchParams();
  let navigate = useNavigate();

  const code = searchParams.get("code");
  const route = searchParams.get("route");

  fetch(`/api/login?code=${code}`)
    .then((resp) => {
      if (resp.status <= 399) {
        return resp.text();
      }
    })
    .then((access_token) => {
      authContext.setAccessToken(access_token!);
      navigate(route!, { replace: true });
    });

  return <div>{code}</div>;
};

const Login = () => {
  const authContext = useAuth();

  const login = async () => {
    authContext.login(authContext);
  };
  const logout = async () => {
    //authService.logout();
  };

  //if (authService.isPending()) {
  //  return <div>Loading...</div>;
  //}

  return (
    <div>
      <button onClick={login}>Login</button>
    </div>
  );
};

export default App;
