import { BrowserRouter, Routes, Route } from "react-router-dom";
import MainLayout from "@/components/layout/MainLayout";
import FrontPage from "@/pages/FrontPage";
import ScenesPage from "@/pages/ScenesPage";
import PerformersPage from "@/pages/PerformersPage";
import StudiosPage from "@/pages/StudiosPage";
import TagsPage from "@/pages/TagsPage";
import GalleriesPage from "@/pages/GalleriesPage";
import ImagesPage from "@/pages/ImagesPage";
import GroupsPage from "@/pages/GroupsPage";
import SettingsPage from "@/pages/SettingsPage";

function App() {
  return (
    <BrowserRouter>
      <MainLayout>
        <Routes>
          <Route path="/" element={<FrontPage />} />
          <Route path="/scenes" element={<ScenesPage />} />
          <Route path="/performers" element={<PerformersPage />} />
          <Route path="/studios" element={<StudiosPage />} />
          <Route path="/tags" element={<TagsPage />} />
          <Route path="/galleries" element={<GalleriesPage />} />
          <Route path="/images" element={<ImagesPage />} />
          <Route path="/groups" element={<GroupsPage />} />
          <Route path="/settings" element={<SettingsPage />} />
        </Routes>
      </MainLayout>
    </BrowserRouter>
  );
}

export default App;
